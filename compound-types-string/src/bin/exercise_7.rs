// Opposite to the seldom using of str, &str and String are used everywhere!

// 🌟🌟 &str can be converted to String in two ways
// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}
