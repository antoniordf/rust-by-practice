// Mutability
// Mutability can be changed when ownership is transferred.

// 🌟
fn main() {
    let s = String::from("hello, ");

    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}
