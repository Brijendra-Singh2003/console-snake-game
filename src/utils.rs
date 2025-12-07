pub fn clear_screen() {
    // print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[3J\x1B[2J\x1B[H");
    // print!("\x1B[2J\x1B[H");
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
