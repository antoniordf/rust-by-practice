// 🌟🌟 We can use String::from or to_string to convert a &str to String
// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s; // Borrow s to get a &str

    println!("Success!");
}

// ANOTHER SOLUTION:
//fn main() {
//    let s = "hello, world"; // This is already a &str, no need to call to_string()
//    let s1: &str = s;

//    println!("Success!");
//}

// EXPLANATION:

//So if you do a reference to a string what you get is &str or just in this case because we forced s1 to have type &str?
// ChatGPT
// When you create a reference to a String in Rust, you indeed get a &str, which is a string slice. The & operator when applied to a String object will automatically coerce the String into a &str, because &String can be dereferenced to &str. This coercion happens due to a feature of Rust called "deref coercion," which allows a reference to a type that implements the Deref trait to be converted to a reference to another type.

// Here's an example to illustrate this:

// fn main() {
//     let s = String::from("hello");
//     let s_slice: &str = &s; // Explicitly typed as &str
//     println!("{}", s_slice); // Prints "hello"
// }
// In this code, &s is a reference to the String variable s, but the type of s_slice is &str. The Rust compiler automatically converts &String to &str for us. This works without explicitly typing s_slice as &str too:

// fn main() {
//     let s = String::from("hello");
//     let s_slice = &s; // Implicitly understood as a &str by the compiler
//     println!("{}", s_slice); // Prints "hello"
// }
// The s_slice variable will still be a &str because Rust understands that when you take a reference to a String, you often want a string slice. This is possible because String implements the Deref trait, which returns a slice of its contained string.

// In your original example, by typing s1: &str, you're explicitly telling the compiler that you want a &str, but even without that type annotation, if you borrow a String, you get a &str.
