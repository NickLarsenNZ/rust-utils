use rust_utils::format::{DebugExt, DisplayExt};

#[derive(Debug, derive_more::Display)]
struct Foo {
    id: String,
}

fn main() {
    let foo = Foo {
        id: "abcdef".into(),
    };

    let debug = foo.format_debug();
    let display = foo.format_display();

    println!("The debug output is: {debug}, and the display output is {display}");
}
