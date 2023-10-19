// Make it work, don't modify `implicitly_ret_unit` !
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

// The function implicitly_ret_unit() does implicitly return the unit type (),
// and the value of v0 is explicitly set to the unit type () as well.

// Here’s a brief breakdown:

// let v0: () = ();: This line initializes a variable v0 with the unit type ().
// let v = (2, 3);: This line seems not to be necessary for the logic of the code, as v is not used in the comparisons or assertions.
// assert_eq!(v0, implicitly_ret_unit()): This assertion checks that the value of v0,
// which is (), is equal to the return value of implicitly_ret_unit(), which is also ().
// So, in essence, the assertion is confirming that both v0 and the return value of implicitly_ret_unit()
// are of the unit type (), making the assertion true and the code run successfully.
