// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
pub mod employees {
    use std::collections::HashMap;
    use std::io;

    pub const DEPARTMENTS: [&str; 5] = ["engineering", "design", "business", "sales", "research"];

    pub const KEY_WORDS: [&str; 5] = ["add", "remove", "print", "end", "get"];

    pub const CURRENT_EMPLOYEES: [&str; 9] = [
        "Add Sally to engineering",
        "Add John to engineering",
        "Add James to design",
        "Add David to design",
        "Add Tedford to design",
        "Add Brian to business",
        "Add Cooper to sales",
        "Add Edward to research",
        "Add Eric to research",
    ];

    pub fn handle_add_case(command: &Vec<&str>) -> Result<String, String> {
        if !DEPARTMENTS.contains(&command[3].to_lowercase().as_str()) {
            return Err(String::from("not a department"));
        }
        return Ok(command[0].to_string());
    }

    pub fn handle_get_case(command: &Vec<&str>) -> Result<String, String> {
        if !DEPARTMENTS.contains(&command[1].to_lowercase().as_str()) {
            return Err(String::from("not a department"));
        }
        return Ok(command[0].to_string());
    }

    pub fn handle_print_case() -> Result<String, String> {
        return Ok("print".to_string());
    }

    pub fn parse_command(command: &Vec<&str>) -> Result<String, String> {
        let command_zero = command[0].to_lowercase();

        if !KEY_WORDS.contains(&command_zero.as_str()) {
            return Err(String::from("not an add command"));
        }

        match command_zero.as_str() {
            "end" => Ok(command_zero),
            "exit" => Ok(command_zero),
            "add" => handle_add_case(command),
            "get" => handle_get_case(command),
            "print" => handle_print_case(),
            _ => Err(String::from("command not found")),
        }
    }

    pub fn add_employee(name: &str, department: &str, company: &mut HashMap<String, Vec<String>>) {
        let target_department = company.get_mut(department);
        match target_department {
            Some(val) => val.push(name.to_string()),
            None => println!("There is no value"),
        }
    }

    pub fn print_department(company: &HashMap<String, Vec<String>>, department: &str) {
        match company.get(&department.to_lowercase()) {
            Some(value) => {
                println!("{:?}", value)
                //
            }
            None => println!("no entry"),
        }
    }

    pub fn print_company(company: &HashMap<String, Vec<String>>) {
        for (key, value) in company {
            println!("{}: {:?}", key, value)
        }
    }

    pub fn make_company() -> HashMap<String, Vec<String>> {
        let mut company = HashMap::new();

        for department in DEPARTMENTS {
            company.insert(String::from(department), Vec::new());
        }

        // add all the current employees
        for command in CURRENT_EMPLOYEES {
            let split_command: Vec<&str> = command.trim().split(" ").collect();
            match parse_command(&split_command) {
                Ok(value) => {
                    if value == "end" {
                        println!("goodbye");
                        break;
                    }
                    add_employee(&split_command[1], &split_command[3], &mut company);
                }
                Err(err) => {
                    println!("{}", err);
                }
            };
        }

        return company;
    }

    pub fn start_cli() {
        let mut company_directory = make_company();

        loop {
            println!("Type 'add employeeName to departmentName' to add an employee");

            let mut command = String::new();

            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            let split_command: Vec<&str> = command.as_str().trim().split(" ").collect();

            match parse_command(&split_command) {
                Ok(value) => match value.as_str() {
                    "end" => {
                        println!("Goodbye");
                        break;
                    }
                    "add" => {
                        add_employee(&split_command[1], &split_command[3], &mut company_directory)
                    }
                    "print" => print_company(&company_directory),
                    "get" => print_department(&company_directory, split_command[1]),
                    _ => println!("not a command"),
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
