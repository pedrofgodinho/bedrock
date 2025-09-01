use crate::println;
use crate::vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

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

#[test_case]
fn test_println_output_multiline() {
    let s = "Some test string that goes beyond the width of the screen. This is some really long test!\nSome test string that does not go beyond the width of the screen.";

    println!("{}", s);

    let mut row = BUFFER_HEIGHT - 4;
    let mut col = 0;
    for c in s.chars() {
        if c == '\n' {
            row += 1;
            col = 0;
            continue;
        }
        let screen_char = WRITER.lock().buffer.chars[row][col].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
        col += 1;
        if col >= BUFFER_WIDTH {
            row += 1;
            col = 0;
        }
    }
}
