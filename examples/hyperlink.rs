use rust_utils::hyperlink::HyperLink;

fn main() {
    let hyperlink =
        HyperLink::new("Display Text", "http://example.com").expect("valid construction");

    println!("{hyperlink}");
}
