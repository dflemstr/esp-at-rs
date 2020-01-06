// TODO: use serial-core directly if it becomes available as a no_std crate
// The types below are taken from the serial-core crate, which is:

// Copyright (c) 2015 David Cuddeback
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/// Serial port baud rates.
///
/// ## Portability
///
/// The `BaudRate` variants with numeric suffixes, e.g., `Baud9600`, indicate standard baud rates
/// that are widely-supported on many systems. While non-standard baud rates can be set with
/// `BaudOther`, their behavior is system-dependent. Some systems may not support arbitrary baud
/// rates. Using the standard baud rates is more likely to result in portable applications.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaudRate {
    /// 110 baud.
    Baud110,

    /// 300 baud.
    Baud300,

    /// 600 baud.
    Baud600,

    /// 1200 baud.
    Baud1200,

    /// 2400 baud.
    Baud2400,

    /// 4800 baud.
    Baud4800,

    /// 9600 baud.
    Baud9600,

    /// 19,200 baud.
    Baud19200,

    /// 38,400 baud.
    Baud38400,

    /// 57,600 baud.
    Baud57600,

    /// 115,200 baud.
    Baud115200,

    /// Non-standard baud rates.
    ///
    /// `BaudOther` can be used to set non-standard baud rates by setting its member to be the
    /// desired baud rate.
    ///
    /// ```no_run
    /// serial_core::BaudOther(4_000_000); // 4,000,000 baud
    /// ```
    ///
    /// Non-standard baud rates may not be supported on all systems.
    BaudOther(usize),
}

impl BaudRate {
    /// Creates a `BaudRate` for a particular speed.
    ///
    /// This function can be used to select a `BaudRate` variant from an integer containing the
    /// desired baud rate.
    ///
    /// ## Example
    ///
    /// ```
    /// # use serial_core::BaudRate;
    /// assert_eq!(BaudRate::Baud9600, BaudRate::from_speed(9600));
    /// assert_eq!(BaudRate::Baud115200, BaudRate::from_speed(115200));
    /// assert_eq!(BaudRate::BaudOther(4000000), BaudRate::from_speed(4000000));
    /// ```
    pub fn from_speed(speed: usize) -> BaudRate {
        match speed {
            110 => BaudRate::Baud110,
            300 => BaudRate::Baud300,
            600 => BaudRate::Baud600,
            1200 => BaudRate::Baud1200,
            2400 => BaudRate::Baud2400,
            4800 => BaudRate::Baud4800,
            9600 => BaudRate::Baud9600,
            19200 => BaudRate::Baud19200,
            38400 => BaudRate::Baud38400,
            57600 => BaudRate::Baud57600,
            115200 => BaudRate::Baud115200,
            n => BaudRate::BaudOther(n),
        }
    }

    /// Returns the baud rate as an integer.
    ///
    /// ## Example
    ///
    /// ```
    /// # use serial_core::BaudRate;
    /// assert_eq!(9600, BaudRate::Baud9600.speed());
    /// assert_eq!(115200, BaudRate::Baud115200.speed());
    /// assert_eq!(4000000, BaudRate::BaudOther(4000000).speed());
    /// ```
    pub fn speed(&self) -> usize {
        match *self {
            BaudRate::Baud110 => 110,
            BaudRate::Baud300 => 300,
            BaudRate::Baud600 => 600,
            BaudRate::Baud1200 => 1200,
            BaudRate::Baud2400 => 2400,
            BaudRate::Baud4800 => 4800,
            BaudRate::Baud9600 => 9600,
            BaudRate::Baud19200 => 19200,
            BaudRate::Baud38400 => 38400,
            BaudRate::Baud57600 => 57600,
            BaudRate::Baud115200 => 115200,
            BaudRate::BaudOther(n) => n,
        }
    }
}

/// Number of bits per character.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CharSize {
    /// 5 bits per character.
    Bits5,

    /// 6 bits per character.
    Bits6,

    /// 7 bits per character.
    Bits7,

    /// 8 bits per character.
    Bits8,
}

/// Parity checking modes.
///
/// When parity checking is enabled (`ParityOdd` or `ParityEven`) an extra bit is transmitted with
/// each character. The value of the parity bit is arranged so that the number of 1 bits in the
/// character (including the parity bit) is an even number (`ParityEven`) or an odd number
/// (`ParityOdd`).
///
/// Parity checking is disabled by setting `ParityNone`, in which case parity bits are not
/// transmitted.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
    /// No parity bit.
    ParityNone,

    /// Parity bit sets odd number of 1 bits.
    ParityOdd,

    /// Parity bit sets even number of 1 bits.
    ParityEven,
}

/// Number of stop bits.
///
/// Stop bits are transmitted after every character.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StopBits {
    /// One stop bit.
    Stop1,

    /// Two stop bits.
    Stop2,
}

/// Flow control modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlowControl {
    /// No flow control.
    FlowNone,

    /// Flow control using XON/XOFF bytes.
    FlowSoftware,

    /// Flow control using RTS/CTS signals.
    FlowHardware,
}
