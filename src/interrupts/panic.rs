use core::panic::PanicInfo;

/// `panic` is called when `micrus` runs into an unexpected error.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
