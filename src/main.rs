use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            departments: HashMap::new(),
        }
    }
    
    fn add_employee(&mut self, name: String, department: String) {
        self.departments
            .entry(department)
            .or_insert_with(Vec::new)
            .push(name);
        
        // Keep the list sorted
        for employees in self.departments.values_mut() {
            employees.sort();
        }
    }
    
    fn get_department_employees(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }
    
    fn get_all_employees_sorted(&self) -> Vec<(String, Vec<String>)> {
        let mut result: Vec<(String, Vec<String>)> = self.departments
            .iter()
            .map(|(dept, employees)| (dept.clone(), employees.clone()))
            .collect();
        
        // Sort by department name
        result.sort_by(|a, b| a.0.cmp(&b.0)); //Glad to know this.
        
        // Ensure each department's employees are sorted (though they should be already)
        for (_, employees) in &mut result {
            employees.sort();
        }
        
        result
    }
}

fn main() {
    let mut company = Company::new();
    println!("Employee Management System");
    println!("Commands:");
    println!("  Add <name> to <department>");
    println!("  List <department>");
    println!("  List all");
    println!("  Quit");
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        if input == "quit" || input == "Quit" {
            break;
        }
        
        if input.starts_with("Add ") && input.contains(" to ") {
            let rest = &input[4..]; // Remove "Add "
            if let Some(to_pos) = rest.find(" to ") {
                let name = &rest[..to_pos];
                let department = &rest[to_pos + 4..];
                
                company.add_employee(name.to_string(), department.to_string());
                println!("Added {} to {}", name, department);
            } else {
                println!("Invalid format. Use 'Add <name> to <department>'");
            }
        } 
        else if input.starts_with("List ") {
            let department = &input[5..];
            
            if department == "all" {
                println!("\nAll employees by department:");
                for (dept, employees) in company.get_all_employees_sorted() {
                    println!("{}: {:?}", dept, employees);
                }
            } else if let Some(employees) = company.get_department_employees(department) {
                println!("Employees in {}: {:?}", department, employees);
            } else {
                println!("Department '{}' not found", department);
            }
        }
        else {
            println!("Unknown command. Try: 'Add Obot to Engineering' or 'List Engineering' or 'List all'");
        }
    }
    
    println!("Goodbye!");
}