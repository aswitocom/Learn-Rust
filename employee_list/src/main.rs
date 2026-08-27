use std::collections::HashMap;
use std::io;

fn main() {
    let mut company_employees: HashMap<String, Vec<String>> = HashMap::new();

    let mut add_employee = String::new();

    println!("Add Employee to a Department");

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

        let employee = words[1].to_string();
        let department = words[3].to_string();

        let department_list = company_employees
            .entry(department)
            .or_insert(Vec::new());

        department_list.push(employee);
        department_list.sort();

        println!("{:?}", company_employees);
    }
}
