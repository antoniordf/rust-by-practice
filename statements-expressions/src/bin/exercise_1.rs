// 🌟🌟
// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2; // This is actually an assignment. We add 2 to the value of x and reassign the variable. Variable assignments are statements so they should end with a semicolon.
        x // We cant return a variable assignment so we explicitly return the value of x so it can be stored in v.
    };

    assert_eq!(v, 3);

    println!("Success!");
}
