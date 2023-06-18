// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
pub mod employees {
    use std::collections::HashMap;
    use std::io;

    pub const DEPARTMENTS: [&str; 5] = ["engineering", "design", "business", "sales", "research"];

    pub const KEY_WORDS: [&str; 2] = ["Add", "Remove"];

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

    pub fn parse_command(command: &str) -> Result<(String, String), String> {
        let split_command: Vec<&str> = command.split(" ").collect();
        if split_command[0].to_lowercase() != "add" {
            return Err(String::from("not an add command"));
        }
        if split_command[0].to_lowercase() == "end" {
            return Ok((String::from("end"), String::from("end")));
        }
        if !DEPARTMENTS.contains(&split_command[3].to_lowercase().as_str()) {
            return Err(String::from("not a department"));
        }

        return Ok((
            String::from(split_command[1]),
            String::from(split_command[3]),
        ));
    }

    pub fn add_employee(name: &str, department: &str, company: &mut HashMap<String, Vec<String>>) {
        let target_department = company.get_mut(department);
        match target_department {
            Some(val) => val.push(name.to_string()),
            None => println!("There is no value"),
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
            match parse_command(command) {
                Ok((name, department)) => {
                    if name == "end" && department == "end" {
                        println!("goodbye");
                        break;
                    }
                    add_employee(&name, &department, &mut company);
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

            match parse_command(command.as_str().trim()) {
                Ok((name, department)) => {
                    add_employee(&name, &department, &mut company_directory);
                    print_company(&company_directory);
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }
}
