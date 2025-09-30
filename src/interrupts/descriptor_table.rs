use crate::interrupts::fault_handlers;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

// lazy_static because the IDT needs to be initialized at runtime,
// not compile-time, but we only want one instance.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // add the various interrupt handlers to the IDT
        idt.breakpoint.set_handler_fn(fault_handlers::breakpoint_handler);
        idt.double_fault.set_handler_fn(fault_handlers::double_fault_handler);
        // idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    };
}

/// `init_idt` initializes the Interrupt Descriptor Table (IDT).
///
/// This function should be called once during early kernel initialization.
pub fn init_idt() {
    // load the static IDT instance into the CPU's IDTR register.
    // after this, the CPU will use the IDT for interrupt lookups.
    IDT.load();
}
