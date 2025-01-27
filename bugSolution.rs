fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements
    if vec.len() > 3 {
        println!("Value at index 3: {}", vec[3]);
    } else {
        println!("Vector doesn't have an element at index 3");
    }
    //Alternative way using get() which returns an Option<&T>  
    match vec.get(3) {
        Some(value) => println!("Value at index 3: {}", value), 
        None => println!("Vector doesn't have an element at index 3"), 
    }
} 