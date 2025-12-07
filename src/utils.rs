use std::io::{self, Write};

pub fn clear_screen() {
    io::stdout().flush().unwrap();
    print!("\x1B[2J\x1B[1;1H");
}
