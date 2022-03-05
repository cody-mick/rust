pub fn run() {
    // Pring to console
    println!("Hello from the print.rs file!");

    // Basic formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Cody", "Idaho");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}.", "Cody", "Idaho", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Cody", activity = "video games");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}