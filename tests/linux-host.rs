#[test]
fn module_revision() -> Result<(), failure::Error> {
    let mut esp32_at = setup()?;
    let revision = nb::block!(esp32_at.get_module_revision())?;
    assert_eq!("", revision.sdk_version);
    assert_eq!("", revision.at_version);
    assert_eq!("", revision.compile_time);
    Ok(())
}

fn setup(
) -> Result<esp32_at::Esp32At<serial_embedded_hal::Rx, serial_embedded_hal::Tx>, failure::Error> {
    let serial = serial_embedded_hal::Serial::new(
        "/dev/ttyUSB1",
        &serial::PortSettings {
            baud_rate: serial::BaudRate::Baud115200,
            char_size: serial::CharSize::Bits8,
            parity: serial::Parity::ParityNone,
            stop_bits: serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        },
    )?;
    let (tx, rx) = serial.split();

    Ok(esp32_at::Esp32At::new(
        rx,
        tx,
        esp32_at::CommandSet::TcpIp | esp32_at::CommandSet::Wifi,
    ))
}
