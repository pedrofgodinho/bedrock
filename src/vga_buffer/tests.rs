use crate::println;
use crate::vga_buffer::{BUFFER_HEIGHT, WRITER};

#[test_case]
fn test_println_simple() {
    println!("test_println output")
}

#[test_case]
fn test_println_many() {
    for i in 0..200 {
        println!("test_println_many output {i}");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
