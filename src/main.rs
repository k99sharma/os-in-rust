// disabling links to standard library
#![no_std] 
#![no_main] // telling rust compiler that we don't want to use the normal entry point chain

// panic implementation
// panic handler defines the function that the compiler should invoke when a panic occurs.
// Standard library provides it's own panic handler, but we disabled it so we need to create our own panic handler.
use core::panic::PanicInfo;     // panic info contains the file and line where the panic happened and the optional panic message.

// function to be called on panic.

// function should never return, so it is marked as a diverging function.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// we are overwriting the OS entry point with our own _start function
#[no_mangle]    // we disable name mangling

pub extern "C" fn _start() -> ! {
    // vga buffer is located at address 0xb8000.
    // each character cell consists of an ASCII bytes and a color byte.
    let vga_buffer = 0xb8000 as *mut u8;    // casting the integer 0xb8000 into a raw pointer

    for (i, b) in b"Hello World".iter().enumerate() {
        // we are using unsafe block to tell rust it is perfectly valid.
        // it doesn't turn off Rust safety check.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = *b;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    loop{}
}