// 🌟🌟
// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u16 + 8; // changed 251_u8 to 251_u16
    let v2 = u8::checked_add(251, 8).unwrap(); // changed i8::checked_add to u8::checked_add
    println!("{},{}", v1, v2);
}
