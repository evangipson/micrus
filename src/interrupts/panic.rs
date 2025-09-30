use crate::println;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

fn show_panic_location(info: &PanicInfo) {
    if let Some(location) = info.location() {
        println!(
            "Location: {} (Line {}, Column {})",
            location.file(),
            location.line(),
            location.column()
        );
    }
}

fn show_panic_message(info: &PanicInfo) {
    if info
        .message()
        .as_str()
        .expect("Could not get kernel panic message")
        .is_empty()
    {
        println!("Message: <no panic message>");
    } else {
        println!("Message: {}", info.message());
    }
}

/// [`panic`] is called when the [`micrus`](crate) kernel runs into an
/// unexpected error. it will show the user information about the panic, like
/// the location it's from and the associated message, and then put the CPU in
/// a low-power state to halt it indefinitely.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC");
    show_panic_location(info);
    show_panic_message(info);
    loop {
        hlt();
    }
}
