use x86_64::{instructions::hlt, structures::idt::InterruptStackFrame};

/// `breakpoint_handler` is the code that runs when an int3 (breakpoint)
/// interrupt is received in `micrus`.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // when a breakpoint interrupt occurs, this function is executed. the `stack_frame`
    // contains information about the CPU state at the time of the interrupt.
    crate::println!("EXCEPTION: BREAKPOINT");
    crate::println!("Stack frame: {:#?}", stack_frame);

    // in a real OS, you might log this, provide a debugger interface, etc.
    // for now, the CPU will resume execution from where the breakpoint was triggered.
}

/// `double_fault_handler` is the code that runs when an exception occurs
/// while `micrus` is handling another exception, and puts the system into
/// an unrecoverable state.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::println!("EXCEPTION: DOUBLE FAULT");
    crate::println!("Stack frame: {:#?}", stack_frame);
    crate::println!("Error Code: {}", error_code);
    crate::println!("An exception occured while trying to handle another exception.");
    crate::println!("System halting.");

    // TODO: don't just halt the CPU indefinitely, show a "blue screen" or
    // try and recover the system via restart or something
    loop {
        hlt();
    }
}
