pub fn run() {
    let name = "Cody";
    let mut age = 25;
    println!("My name is {} and I am {} years old", name, age);

    age = 26;
    
    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Cody", 25);
    println!("{} is {}", my_name, my_age);
}