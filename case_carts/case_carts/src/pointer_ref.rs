pub fn run() {
    // Primitive Array
    let array1 = [1, 2, 3];
    let array2 = array1;

    // Vectors
    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;

    println!("Values: {:?}", (&vector1, vector2));
}