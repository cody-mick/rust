use std::io;

struct Doctor {
    name: String,
    specialty: String,
    personality: String,
    extra_tools: i32
}

impl Doctor {
    fn new(name: &str, specialty: &str, personality: &str, extra_tools: i32) -> Doctor {
        Doctor {
            name: name.to_string(),
            specialty: specialty.to_string(),
            personality: personality.to_string(),
            extra_tools: extra_tools
        }
    }

    fn update_tools(&mut self, extra_tools: i32){
        self.extra_tools = extra_tools;
    }
}

struct Case {
    name: String,
    tools: Vec<i32>
}

impl Case {
    fn new(name: &str, tools: Vec<i32>) -> Case {
        Case {
            name: name.to_string(),
            tools: tools
        }
    }
}

fn main(){

    // List of starting doctors
    let mut neuro = Doctor::new("Stephen Strange", "Neurosurgery", "a perfectionist, but a prodigy", 34576);

    let  mut ortho = Doctor::new("Duke Crackems", "Orthopedics", "aggressive, but effective", 76859);

    let mut general = Doctor::new("Jack Alltrades", "General", "alright at everything", 90876);

    // List of starting cases
    let appendectomy = Case::new("Appendectomy", vec![14356, 65787, 22340]);
    let crainiotomoy = Case::new("Crainiotomy", vec![45365, 99099, 65765]);
    let femoral_nail = Case::new("Femoral Nail", vec![44354, 66543, 13242]);

    // Initialize variable for doctor and case selection
    let mut case = String::new();
    let mut menu_option = String::new();
    let mut doctor_pref = String::new();

    // Choose a case
    println!("Case Cart System");

    println!("What would you like to do?");
    println!("1. Prepare for a case");
    println!("2. Edit Doctor preferences");
    println!("3. Quit");
    
    io::stdin()
        .read_line(&mut menu_option)
        .expect("Please pick a choice from the menu");

        let menu_option: i32 = menu_option.trim().parse().expect("Please input a number");

        if menu_option == 1 {
            println!("Please select the case you are preparing for: ");
            println!("1. {}", appendectomy.name);
            println!("2. {}", crainiotomoy.name);
            println!("3. {}", femoral_nail.name);

            io::stdin()
                .read_line(&mut case)
                .expect("Failed to recognize case");
        
            // Convert the stdin to an integer
            let case: i32 = case.trim().parse().expect("Please input a number.");
        
            // Tell the user the ID number of the doctor's specific tool set, alongside the tools already needed for the case
            if case == 1 {
                println!("You will be working with Dr. {}. They are {}. Their specialty is {}", general.name, general.personality, general.specialty);
                println!("You will need to gather the tool cases with the ID numbers of {:?}", appendectomy.tools);
                println!("Dr. {} also has a personal preferred toolset of case number: {}", general.name, general.extra_tools)
            } else if case == 2 {
                println!("You will be working with Dr. {}. They are {}. Their specialty is {}", neuro.name, neuro.personality, neuro.specialty);
                println!("You will need to gather the tool cases with the ID numbers of {:?}", crainiotomoy.tools);
                println!("Dr. {} also has a personal preferred toolset of case number: {}", neuro.name, neuro.extra_tools)
            } else if case == 3 {
                println!("You will be working with Dr. {}. They are {}. Their specialty is {}", ortho.name, ortho.personality, ortho.specialty);
                println!("You will need to gather the tool cases with the ID numbers of {:?}", femoral_nail.tools);
                println!("Dr. {} also has a personal preferred toolset of case number: {}", ortho.name, ortho.extra_tools)
            } else {
                println!("Case not recognized");
            }
        } else if menu_option == 2 {
            println!("Which doctor would you like to update preferences for?");
            println!("1. {}", general.name);
            println!("2. {}", neuro.name);
            println!("3. {}", ortho.name);

            io::stdin()
                .read_line(&mut doctor_pref)
                .expect("Please select a choice from the menu");

            let doctor_pref: i32 = doctor_pref.trim().parse().expect("Please enter a number");

            // Update doctor preferences using the class function update_tools
            if doctor_pref == 1 {
                println!("Dr. {} currently has a extra tools case ID of: {}", general.name, general.extra_tools);
                let mut new_id = String::new();
                println!("Please enter the 5 digit ID number for the new preferred tool set: ");

                io::stdin()
                    .read_line(&mut new_id)
                    .expect("Please enter the 5 digit ID number.");

                let new_id: i32 = new_id.trim().parse().expect("Please enter a number");

                general.update_tools(new_id);

                println!("The toolset has been successfully updated!");
                println!("Dr. {} preferred toolset ID is now: {}", general.name, general.extra_tools);
                
            } else if doctor_pref == 2 {

                println!("Dr. {} currently has a extra tools case ID of: {}", neuro.name, neuro.extra_tools);
                let mut new_id = String::new();
                println!("Please enter the 5 digit ID number for the new preferred tool set: ");

                io::stdin()
                    .read_line(&mut new_id)
                    .expect("Please enter the 5 digit ID number.");

                let new_id: i32 = new_id.trim().parse().expect("Please enter a number");

                neuro.update_tools(new_id);

                println!("The toolset has been successfully updated!");
                println!("Dr. {} preferred toolset ID is now: {}", neuro.name, neuro.extra_tools);

            } else if doctor_pref == 3 {

                println!("Dr. {} currently has a extra tools case ID of: {}", ortho.name, ortho.extra_tools);
                let mut new_id = String::new();
                println!("Please enter the 5 digit ID number for the new preferred tool set: ");

                io::stdin()
                    .read_line(&mut new_id)
                    .expect("Please enter the 5 digit ID number.");

                let new_id: i32 = new_id.trim().parse().expect("Please enter a number");

                ortho.update_tools(new_id);

                println!("The toolset has been successfully updated!");
                println!("Dr. {} preferred toolset ID is now: {}", ortho.name, ortho.extra_tools);

            } else {
                println!("Please select a doctor from the menu")
            }
        }
}