use crate::serial;

#[derive(Debug)]
pub struct State {
    module_revision: ModuleRevision,
    current_uart_config: UartConfig,
    default_uart_config: UartConfig,
}

#[derive(Debug)]
pub struct ModuleRevision {
    pub at_version: heapless::String<heapless::consts::U64>,
    pub sdk_version: heapless::String<heapless::consts::U64>,
    pub compile_time: heapless::String<heapless::consts::U64>,
}

#[derive(Debug)]
pub struct UartConfig {
    baud_rate: serial::BaudRate,
    char_size: serial::CharSize,
    stop_bits: serial::StopBits,
    parity: serial::Parity,
    flow_control: serial::FlowControl,
}
