/// The commonly used I/O port for ACPI power management status/command.
pub const ACPI_SHUTDOWN_PORT: u16 = 0x604;

/// The commonly used debug I/O port for power management.
pub const DEBUG_SHUTDOWN_PORT: u16 = 0x501;

/// The old-school QEMU/Bochs power-off port.
pub const POWER_OFF_PORT: u16 = 0xf4;

/// Often triggers an immediate shutdown in VirtualBox and QEMU.
pub const VB_SHUTDOWN_VALUE: u32 = 0x2000;

/// Often triggers an immediate shutdown in many environments.
pub const IMMEDIATE_SHUTDOWN_VALUE: u32 = 0x0001;

/// Represents any command, just an empty 8-bit message.
pub const EMPTY_BYTE: u32 = 0x00;
