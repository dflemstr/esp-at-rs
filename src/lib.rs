#![no_std]

use core::fmt;

mod parser;
mod serial;
mod state;

#[derive(Debug)]
pub struct Esp32At<RX, TX>
where
    RX: embedded_hal::serial::Read<u8>,
    TX: embedded_hal::serial::Write<u8>,
{
    rx: RX,
    tx: TX,
    command_sets: enumset::EnumSet<CommandSet>,
}

#[derive(Debug, enumset::EnumSetType)]
pub enum CommandSet {
    // Taken from https://www.espressif.com/sites/default/files/documentation/esp32_at_instruction_set_and_examples_en.pdf
    Wifi,
    TcpIp,
    Ble,
    // Taken from https://github.com/particle-iot/argon-ncp-firmware/blob/master/README.md
    ParticleArgonExt,
}

#[derive(Debug, failure::Fail)]
pub enum Error<RXE, TXE>
where
    RXE: failure::Fail,
    TXE: failure::Fail,
{
    #[fail(display = "command set not supported: {:?}", command_set)]
    CommandSetNotSupported { command_set: CommandSet },
    #[fail(display = "unexpected response")]
    UnexpectedResponse,
    #[fail(display = "buffer overflow")]
    BufferOverflow,
    #[fail(display = "UART read error")]
    UartRead {
        #[cause]
        cause: RXE,
    },
    #[fail(display = "UART write error")]
    UartWrite {
        #[cause]
        cause: TXE,
    },
    #[fail(display = "UTF-8 decoding error: {}", cause)]
    Utf8 {
        // can't use #[cause] since the Fail trait is not implemented
        cause: core::str::Utf8Error,
    },
}

struct Writer<'a, RX, TX>
where
    RX: embedded_hal::serial::Read<u8>,
    RX::Error: failure::Fail,
    TX: embedded_hal::serial::Write<u8>,
    TX::Error: failure::Fail,
{
    this: &'a mut Esp32At<RX, TX>,
    error_ref: &'a mut Option<nb::Error<Error<RX::Error, TX::Error>>>,
}

macro_rules! write_command {
    ($this:expr, $template:expr) => {
        write_command!($this, $template,)
    };
    ($this:expr, $template:expr, $($args:tt)*) => {
        $this.write_command(format_args!(concat!($template, "\r\n"), $($args)*))
    }
}

impl<RX, TX> Esp32At<RX, TX>
where
    RX: embedded_hal::serial::Read<u8>,
    RX::Error: failure::Fail,
    TX: embedded_hal::serial::Write<u8>,
    TX::Error: failure::Fail,
{
    pub fn new(rx: RX, tx: TX, command_sets: enumset::EnumSet<CommandSet>) -> Self {
        Self {
            rx,
            tx,
            command_sets,
        }
    }

    pub fn test_startup(&mut self) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        write_command!(self, "AT")?;
        self.expect_ok_response()
    }

    pub fn restart(&mut self) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        write_command!(self, "AT+RST")?;
        self.expect_ok_response()
    }

    pub fn get_module_revision(
        &mut self,
    ) -> nb::Result<state::ModuleRevision, Error<RX::Error, TX::Error>> {
        write_command!(self, "AT+GMR")?;

        let at_version = self.read_line()?;
        let sdk_version = self.read_line()?;
        let compile_time = self.read_line()?;

        self.expect_ok_response()?;

        Ok(state::ModuleRevision {
            at_version,
            sdk_version,
            compile_time,
        })
    }

    pub fn enter_deep_sleep(
        &mut self,
        wakeup_delay_ms: u32,
    ) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        write_command!(self, "AT+GSLP={}", wakeup_delay_ms)?;
        self.ignore_line()?;
        self.expect_ok_response()
    }

    pub fn factory_reset(&mut self) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        write_command!(self, "AT+RESTORE")?;
        self.expect_ok_response()
    }

    fn expect_ok_response(&mut self) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        self.expect_response("OK")
    }

    fn expect_response(&mut self, response: &str) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        self.expect(response.as_bytes())
    }

    fn expect(&mut self, data: &[u8]) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        for &expected_byte in data {
            let actual_byte = self.getc()?;

            if expected_byte != actual_byte {
                return Err(nb::Error::Other(Error::UnexpectedResponse));
            }
        }
        Ok(())
    }

    fn read_line<N>(&mut self) -> nb::Result<heapless::String<N>, Error<RX::Error, TX::Error>>
    where
        N: heapless::ArrayLength<u8>,
    {
        let mut last = [0; 2];
        let mut result = heapless::Vec::new();

        loop {
            last[0] = last[1];
            last[1] = self.getc()?;
            if last == [b'\r', b'\n'] {
                break;
            }

            result.push(last[0]).or(Err(Error::BufferOverflow))?;
        }

        Ok(heapless::String::from_utf8(result).map_err(|cause| Error::Utf8 { cause })?)
    }

    fn ignore_line(&mut self) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        let mut last = [0; 2];
        while last != [b'\r', b'\n'] {
            last[0] = last[1];
            last[1] = self.getc()?;
        }
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        for &byte in data {
            self.putc(byte)?;
        }
        Ok(())
    }

    fn write_command(
        &mut self,
        command: fmt::Arguments,
    ) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        use core::fmt::Write;

        let mut error = None;
        let error_ref = &mut error;

        (Writer {
            this: self,
            error_ref,
        })
        .write_fmt(command)
        .map_err(|_| {
            error
                .take()
                .expect("write failed but writer did not preserve the underlying error")
        })
    }

    fn getc(&mut self) -> nb::Result<u8, Error<RX::Error, TX::Error>> {
        self.rx
            .read()
            .map_err(|nb| nb.map(|cause| Error::UartRead { cause }))
    }

    fn putc(&mut self, byte: u8) -> nb::Result<(), Error<RX::Error, TX::Error>> {
        self.tx
            .write(byte)
            .map_err(|nb| nb.map(|cause| Error::UartWrite { cause }))
    }
}

impl<'a, RX, TX> fmt::Write for Writer<'a, RX, TX>
where
    RX: embedded_hal::serial::Read<u8>,
    RX::Error: failure::Fail,
    TX: embedded_hal::serial::Write<u8>,
    TX::Error: failure::Fail,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if let Err(err) = self.this.putc(byte) {
                *self.error_ref = Some(err);
                return Err(fmt::Error);
            }
        }
        Ok(())
    }
}
