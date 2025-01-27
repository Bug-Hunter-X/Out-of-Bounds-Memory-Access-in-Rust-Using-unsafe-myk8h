fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // This is the problematic line
    unsafe {
        let val = *ptr.add(3);  // Accessing memory out of bounds
        println!("Value at index 3: {}", val);
    }
}