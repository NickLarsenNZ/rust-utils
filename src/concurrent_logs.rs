// color wheel for getting next color for a new id

fn main() {
    for i in 30..=37 {
        print!("\x1b[0;{i}mHello\x1b[0m")
    }
}
