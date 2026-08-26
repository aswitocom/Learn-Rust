use std::collections::HashMap;
use std::io;

fn main() {
    let mut company_employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut employees = Vec::new();

    let mut add_employee = String::new();

    println!("Add Employee to a Department");
    io::stdin()
        .read_line(&mut add_employee)
        .expect("Failed to capture");

    let words: Vec<&str> = add_employee.split_whitespace().collect();

    let employee= words[1].to_string();
    let department = words[3].to_string();

    company_employees.entry(department.clone()).or_insert(Vec::new()).push(employee);

    employees.push(company_employees);

    println!("{:?}", employees);
}
