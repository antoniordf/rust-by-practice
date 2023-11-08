// 🌟🌟 You can only concat a String with &str, and String's ownership can be moved to another variable.
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // When we call + operator, the compiler uses the add method in the Add trait, which is implemented for String and &str. The signature of the add method looks like this:
                       // fn add(self, s: &str) -> String;
                       // This method takes ownership of self, because in the case of String it is necessary in order to be able to modify self. This means we can't use s1 after the addition. At the same time, the add function
                       // takes a reference to a string slice for the second parameter, which is why we use &s2.
    assert_eq!(s3, "hello,world!");
    println!("{}", s2);
}
