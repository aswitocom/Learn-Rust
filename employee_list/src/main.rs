use std::collections::HashMap;
use std::io;

fn main() {
    let mut company_employees: HashMap<String, Vec<String>> = HashMap::new();

    let mut add_employee = String::new();

    println!("Add Employee to a Department (e.g., 'Add Sally to Engineering");

    loop {
        add_employee.clear();

        io::stdin()
            .read_line(&mut add_employee)
            .expect("Failed to capture");

        let add_employee = add_employee.trim();

        if add_employee.to_lowercase() == "exit" {
            break;
        }

        let words: Vec<&str> = add_employee.split_whitespace().collect();

        let employee = match words.get(1) {
            Some(emp) => emp.to_string(),
            None => {
                println!("Error: Missing employee name.");
                continue;
            }
        };

        let department = match words.get(3) {
            Some(dept) => dept.to_string(),
            None => {
                println!("Error: Missing department name.");
                continue;
            }
        };

        let department_list = company_employees.entry(department).or_insert(Vec::new());

        department_list.push(employee);
        department_list.sort();

        println!("{:?}", company_employees);
    }
}
