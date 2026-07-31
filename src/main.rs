#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ferrumos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ferrumos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ferrumos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ferrumos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ferrumos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ferrumos::test_panic_handler(info)
}
