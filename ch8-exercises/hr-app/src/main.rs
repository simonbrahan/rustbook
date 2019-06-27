use std::collections::HashMap;
use std::io;

struct DataStore {
    departments: HashMap<String, Vec<String>>,
}

impl DataStore {
    fn new() -> DataStore {
        DataStore { departments: HashMap::new() }
    }

    fn run_instruction(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Quit => quit_register(),
            Instruction::AddDepartment(department) => self.add_department(&department),
            Instruction::ListDepartments => self.list_departments(),
            _ => true,
        }
    }

    fn add_department(&mut self, department: &str) -> bool {
        if self.departments.contains_key(department) {
            println!("Department {} already exists\n", department);
        } else {
            self.departments.insert(department.to_string(), Vec::new());

            println!("Department {} added\n", department);
        }

        true
    }

    fn list_departments(&self) -> bool {
        let mut departments: Vec<&String> = self.departments.keys().collect();
        departments.sort();

        println!("Listing departments:\n");

        for department in departments {
            println!("{}", department);
        }

        println!("\nEnd of department list\n");

        true
    }
}

enum Instruction {
    AddStaffMember { name: String, department: String },
    AddDepartment(String),
    ListByDepartment(String),
    ListAll,
    ListDepartments,
    Quit,
}

fn main() {
    println!("HR register");

    let mut data = DataStore::new();

    loop {
        let instruction = get_instruction();
        if !data.run_instruction(instruction) {
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

        if instruction_string == "list departments" {
            return Instruction::ListDepartments;
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
                list departments
                quit"
        );
    }
}

fn quit_register() -> bool {
    false
}
