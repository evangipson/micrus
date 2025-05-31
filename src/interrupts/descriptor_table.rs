use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// lazy_static because the IDT needs to be initialized at runtime,
// not compile-time, but we only want one instance.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // 0x03 is the breakpoint interrupt vector
        // set_handler_fn sets the handler for this interrupt.
        // Handlers must have the 'x86-interrupt' calling convention.
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // TODO: Add more handlers here in future steps!
        // idt.double_fault.set_handler_fn(double_fault_handler);
        // idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

/// Initializes the Interrupt Descriptor Table (IDT).
///
/// This function should be called once during early kernel initialization.
pub fn init_idt() {
    // load the static IDT instance into the CPU's IDTR register.
    // after this, the CPU will use the IDT for interrupt lookups.
    IDT.load();
}

/// Handler for the Breakpoint (INT 3) interrupt.
///
/// The `x86-interrupt` calling convention is crucial here. It tells the Rust
/// compiler to generate code that saves and restores all general-purpose
/// registers, and handles the error code (if any) pushed by the CPU for
/// certain exceptions.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // when a breakpoint interrupt occurs, this function is executed. the `stack_frame`
    // contains information about the CPU state at the time of the interrupt.
    crate::println!("EXCEPTION: BREAKPOINT");
    crate::println!("Stack frame: {:#?}", stack_frame);

    // in a real OS, you might log this, provide a debugger interface, etc.
    // for now, the CPU will resume execution from where the breakpoint was triggered.
}

// TODO: add more handlers like:
/*
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64, // Double faults always push an error code
) -> ! { // A double fault handler should never return, hence '!'
    crate::println!("EXCEPTION: DOUBLE FAULT");
    crate::println!("Stack frame: {:#?}", stack_frame);
    crate::println!("Error code: {}", error_code);
    loop { crate::hlt(); } // Halt the CPU indefinitely
}
*/
