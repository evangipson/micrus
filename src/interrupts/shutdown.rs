use crate::interrupts::constants;
use x86_64::instructions::port::Port;

/// Initiates a port shutdown.
///
/// Note: this function will never return, denoted by the '!' return type.
pub fn shutdown() {
    // attempt standard ACPI shutdown power command
    unsafe {
        Port::<u32>::new(constants::ACPI_SHUTDOWN_PORT).write(constants::EMPTY_BYTE);
    }

    // attempt debug port shutdown command
    unsafe {
        Port::<u32>::new(constants::DEBUG_SHUTDOWN_PORT).write(constants::EMPTY_BYTE);
    }

    // attempt old-school power-off message with an empty byte (last resort)
    unsafe {
        Port::<u32>::new(constants::POWER_OFF_PORT).write(constants::EMPTY_BYTE);
    }

    // if the shutdown I/O write fails (which it often does silently),
    // halt the CPU permanently to prevent garbage execution.
    x86_64::instructions::hlt();
}
