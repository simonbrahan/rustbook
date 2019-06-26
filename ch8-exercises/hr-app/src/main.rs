use std::collections::HashMap;
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

    let mut data = HashMap::new();

    loop {
        let instruction = get_instruction();
        println!("{:?}", instruction);
        if !run_instruction(&mut data, instruction) {
            println!("Goodbye");
            break;
        }
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

        if instruction_parts.len() == 2 && instruction_parts[0] == "list" {
            return Instruction::ListByDepartment(instruction_parts[1].to_string());
        }

        if instruction_parts.len() == 3
            && instruction_parts[0] == "add"
            && instruction_parts[1] == "department"
        {
            return Instruction::AddDepartment(instruction_parts[2].to_string());
        }

        if instruction_parts.len() == 4
            && instruction_parts[0] == "add"
            && instruction_parts[2] == "to"
        {
            return Instruction::AddStaffMember {
                name: instruction_parts[1].to_string(),
                department: instruction_parts[3].to_string(),
            };
        }

        println!(
            "Invalid instruction. Valid instructions:
                add STAFF NAME to DEPARTMENT NAME
                add department DEPARTMENT name
                list all
                list DEPARTMENT NAME
                quit"
        );
    }
}

fn run_instruction(data: &mut HashMap<String, Vec<String>>, instruction: Instruction) -> bool {
    match instruction {
        Instruction::Quit => false,
        _ => true,
    }
}
