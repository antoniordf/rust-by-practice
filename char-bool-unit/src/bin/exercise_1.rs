// Make it work

// The code is trying to assert the size of individual characters.
// In Rust, characters (char) are Unicode scalar values and always occupy 4 bytes.
// Hence, irrespective of whether the character is ASCII or non-ASCII (like '中'), the size_of_val will return 4.
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    // Characters in Rust always occupy 4 bytes
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    // Even non-ASCII characters like '中' occupy 4 bytes as well
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
