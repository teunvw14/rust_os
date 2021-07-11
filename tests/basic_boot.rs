#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::run_tests)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use rust_os::vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// Tests

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

// #[test_case]
// fn test_println_output() {
//     let s = "Some test string that fits on a single line.";
//     println!("{}", s);
//     for (i, c) in s.chars().enumerate() {
//         let screen_char = vga_buffer::WRITER.lock().buffer.chars[vga_buffer::BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character), c);
//     }
// }


#[test_case]
fn test_println_long() {
    println!("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean enim justo, egestas vitae vehicula non, tempor condimentum ligula. In urna felis, convallis non turpis id, varius suscipit tortor. Aliquam eu velit at velit pellentesque vulputate. Donec sit amet sem sit amet elit porttitor euismod. Nullam lectus tellus, vulputate at nibh sed, hendrerit pharetra ante. Curabitur auctor faucibus mi, sit amet ultrices turpis dapibus id. Fusce leo ipsum, iaculis ut mi ut, consequat aliquam purus. Nunc quis imperdiet magna. Sed semper tellus id ante sollicitudin, eu viverra lacus hendrerit. Quisque in augue tincidunt, viverra turpis in, tristique nibh. Pellentesque tincidunt, quam id ultrices sodales, elit lectus malesuada magna, non pharetra sem sem a ipsum. Vivamus venenatis diam nec risus posuere laoreet elementum ac elit. Nullam consectetur ante at ex bibendum, sit amet pretium elit blandit. Etiam sem dolor, iaculis a placerat sed, auctor non nisi.")
}