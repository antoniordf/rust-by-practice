// 🌟🌟🌟 Sometimes there are just too many characters that need to be escaped or it's just much more convenient to write a string out as-is. This is where raw string literals come into play.
/* Fill in the blank and fix the errors */
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = __;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
