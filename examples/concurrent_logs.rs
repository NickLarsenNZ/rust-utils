// color wheel for getting next color for a new id

use std::cmp::max;

fn main() {
    for i in 0..=229 {
        let color = (i * (i + 3)) % 229;
        // let color = i;
        println!("\x1b[38;5;{color}m{color}\x1b[0m")
    }

    let c = "🤦";
    println!("{c} = {x}, {y}", x = c.len(), y = c.char_indices().count())
}

struct Logger {
    id_width: usize,
    wrap: bool,
}

impl Logger {
    fn new() -> Self {
        Self {
            // This will be calculated during each source registration
            id_width: 0,

            //
            wrap: false,
        }
    }

    /// Register a log source
    fn register(&mut self, id: impl AsRef<str>) {
        self.id_width = max(id.as_ref().char_indices().count(), self.id_width);
    }

    /// Write a log for a given log source by id
    fn write(id: impl AsRef<str>, log: impl AsRef<str>) {}
}
