// main.rs

#![no_main]
#![no_std]

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("asdfn asdofn\n").unwrap();
    // write!(vga_buffer::WRITER.lock(), "{:x} {}\n", 42, 3.1415).unwrap();

    println!("Hello World{}", "!");
    panic!("Some panic message");

    // loop {}
}


// Deal with panics 
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
