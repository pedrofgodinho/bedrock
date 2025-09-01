#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bedrock::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bedrock::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    bedrock::init();

    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

fn main() {
    println!("Hello World{}", "!");
    println!(
        "This is an example of a really long line that should move on to the next line and stuff"
    );

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    println!("Finished");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bedrock::test_panic_handler(info)
}
