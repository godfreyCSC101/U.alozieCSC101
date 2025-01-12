
use std::io;

 fn main() {
    println(   

struct Patient {
    name: String,
    dob: String,
    email: String,
    phone: String,
    siblings: i32,
    children: i32,
    diagnosis: String,
    village: String,
}

fn main() {
    let mut patients = Vec::new();

    loop {
        println!("1. Register Patient");
        println!("2. Display Patient Info");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter patient's name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("Enter patient's date of birth (YYYY-MM-DD): ");
                let mut dob = String::new();
                io::stdin().read_line(&mut dob).unwrap();

                println!("Enter patient's email: ");
                let mut email = String::new();
                io::stdin().read_line(&mut email).unwrap();

                println!("Enter patient's phone number: ");
                let mut phone = String::new();
                io::stdin().read_line(&mut phone).unwrap();

                println!("Enter number of siblings: ");
                let mut siblings = String::new();
                io::stdin().read_line(&mut siblings).unwrap();
                let siblings: i32 = siblings.trim().parse().unwrap();

                println!("Enter number of children: ");
                let mut children = String::new();
                io::stdin().read_line(&mut children).unwrap();
                let children: i32 = children.trim().parse().unwrap();

                println!("Enter diagnosis: ");
                let mut diagnosis = String::new();
                io::stdin().read_line(&mut diagnosis).unwrap();

                println!("Enter village of residence: ");
                let mut village = String::new();
                io::stdin().read_line(&mut village).unwrap();

                let patient = Patient {
                    name: name.trim().to_string(),
                    dob: dob.trim().to_string(),
                    email: email.trim().to_string(),
                    phone: phone.trim().to_string(),
                    siblings,
                    children,
                    diagnosis: diagnosis.trim().to_string(),
                    village: village.trim().to_string(),
                };

                patients.push(patient);

                println!("Patient registered successfully.");
            }
            "2" => {
                println!("Enter patient's name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                for patient in &patients {
                    if patient.name == name.trim() {
                        println!("Name: {}", patient.name);
                        println!("Date of Birth: {}", patient.dob);
                        println!("Email: {}", patient.email);
                        println!("Phone: {}", patient.phone);
                        println!("Siblings: {}", patient.siblings);
                        println!("Children: {}", patient.children);
                        println!("Diagnosis: {}", patient.diagnosis);
                        println!("Village: {}", patient.village);
                    }
                }
            }
            "3" => break,
            _ => println!("Invalid choice. Please choose again."),
        }
    }
}
)
}