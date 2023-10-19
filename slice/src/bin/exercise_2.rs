// A slice reference is a two-word object, for simplicity reasons, from now on we will use slice instead of slice reference.
// The first word is a pointer to the data, and the second word is the length of the slice.
// The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64.
// Slices can be used to borrow a section of an array, and have the type signature &[T].

fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    // I changed the value 8 to 16 because the size of slice metadata typically consists of a pointer and a length,
    // each being the size of a usize (which is usually 8 bytes on a 64-bit system).
    // Therefore, 8 (pointer) + 8 (length) = 16. Note that the actual size might vary depending on the system architecture (32-bit or 64-bit).
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
