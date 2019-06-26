use std::io;

#[derive(Debug)]
enum Instruction {
    AddStaffMember { name: String, department: String },
    AddDepartment(String),
    ListByDepartment(String),
    ListAll,
    Quit,
}

fn main() {
    println!("HR register");

    loop {
        let instruction = get_instruction();
        println!("{:?}", instruction);
    }
}

fn get_instruction() -> Instruction {
    loop {
        println!("Please enter instruction");

        let mut instruction_string = String::new();

        io::stdin()
            .read_line(&mut instruction_string)
            .expect("Failed to read instruction");

        let instruction_string = instruction_string.trim();

        if instruction_string == "quit" {
            return Instruction::Quit;
        }

        if instruction_string == "list all" {
            return Instruction::ListAll;
        }

        let instruction_parts: Vec<&str> = instruction_string.split_whitespace().collect();

        if instruction_parts.len() == 2
            && instruction_parts[0] == "list"
            && is_department(&instruction_parts[1].to_string())
        {
            return Instruction::ListByDepartment(instruction_parts[1].to_string());
        }
    }
}

fn is_department(department_name: &str) -> bool {
    true
}
