use x86_64::instructions::port::PortReadOnly;

// ps/2 keyboard ports
const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

// status register bits
// bit 0: output buffer full (data is available)
const STATUS_OUTPUT_BUFFER_FULL: u8 = 0x01;

// Scan Code Set 1 (Simplified for common keys)
// This is a very incomplete mapping. You'd need a much larger one for a full keyboard.
// It maps basic scan codes to their ASCII representation for typical QWERTY layout.
fn scancode_to_ascii(scancode: u8) -> Option<char> {
    match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0A => Some('9'),
        0x0B => Some('0'),
        0x10 => Some('q'),
        0x11 => Some('w'),
        0x12 => Some('e'),
        0x13 => Some('r'),
        0x14 => Some('t'),
        0x15 => Some('y'),
        0x16 => Some('u'),
        0x17 => Some('i'),
        0x18 => Some('o'),
        0x19 => Some('p'),
        0x1E => Some('a'),
        0x1F => Some('s'),
        0x20 => Some('d'),
        0x21 => Some('f'),
        0x22 => Some('g'),
        0x23 => Some('h'),
        0x24 => Some('j'),
        0x25 => Some('k'),
        0x26 => Some('l'),
        0x2C => Some('z'),
        0x2D => Some('x'),
        0x2E => Some('c'),
        0x2F => Some('v'),
        0x30 => Some('b'),
        0x31 => Some('n'),
        0x32 => Some('m'),
        0x39 => Some(' '),  // Spacebar
        0x1C => Some('\n'), // Enter key
        // Add more common keys as needed (e.g., Shift, Caps Lock, special characters)
        // For shift, you'd need to track the shift key state.
        _ => None, // Unknown or non-printable scancode
    }
}

/// Reads a single character from the PS/2 keyboard, blocking until a key is pressed.
///
/// This uses busy-waiting (polling) and is inefficient. A real OS would use interrupts.
pub fn read_char() -> char {
    let mut data_port = PortReadOnly::<u8>::new(DATA_PORT);
    let mut status_port = PortReadOnly::<u8>::new(STATUS_PORT);

    loop {
        // SAFETY: Reading from I/O ports is inherently unsafe as it interacts with hardware.
        // We trust the port addresses are correct for the PS/2 controller.
        let status = unsafe { status_port.read() };

        // Check if the output buffer is full (meaning new data is available)
        if status & STATUS_OUTPUT_BUFFER_FULL != 0 {
            // Read the scancode
            let scancode = unsafe { data_port.read() };

            // We only care about key *presses* (not releases) for this simple example.
            // Press scancodes are usually < 0x80. Release scancodes are scancode + 0x80.
            if scancode < 0x80
                && let Some(c) = scancode_to_ascii(scancode)
            {
                return c;
            }
        }
        // If no data, loop and try again (busy-wait)
    }
}
