use std::io;

struct Doctor {
    name: String,
    specialty: String
}

impl Doctor {
    fn new(name: &str, specialty: &str) -> Doctor {
        Doctor {
            name: name.to_string(),
            specialty: specialty.to_string()
        }
    }
}

fn main(){

    // List of starting doctors
    let neurosurgeon = Doctor::new("John Doe", "Neurosurgeon");
    println!("Doctor: {}, Specialty: {}", neurosurgeon.name, neurosurgeon.specialty);

    println!("Case Cart System");
    println!("Please input the case you are preparing for: ");

    let mut case = String::new();
    let mut doctor = String::new();

    io::stdin()
        .read_line(&mut case)
        .expect("Failed to recognize case");
    
    println!("Please input the doctor you are working with: ");
    io::stdin()
        .read_line(&mut doctor)
        .expect("Failed to recognize doctor");

    println!("Your selected case is: {}", case);
    println!("Your selected doctor is: {}", doctor);
}