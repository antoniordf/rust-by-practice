fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];
    println!("{}", slice);

    assert!(slice == "你");

    println!("Success!");
}

// UTF-8 Encoding: Characters in the string "你好，世界" are encoded in UTF-8, and each character occupies multiple bytes.
// For instance, "你" is represented using 3 bytes in UTF-8.

// Slicing: When you slice a string in Rust, the indices you use refer to bytes, not characters.
// So, &s[0..2] would try to slice the string from the 1st byte to the 2nd, but this does not give you a valid
// UTF-8 character because "你" actually occupies bytes 0 to 2.

// By adjusting the slice to &s[0..3], we correctly capture the bytes representing
// the character "你", allowing the code to run successfully.
