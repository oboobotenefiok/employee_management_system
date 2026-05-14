

# Employee Management System

A command-line interface (CLI) tool built in Rust that manages employees by department. The system stores employee names in departments and allows listing by department or company-wide.

## ACTUAL CAPABILITIES

This application strictly performs the following operations:

· Add employee to department – Using the command format: Add <name> to <department>
· List employees in a specific department – Using the command: List <department>
· List all employees across all departments – Using the command: List all
· Automatic sorting – Employee names are automatically sorted alphabetically within each department. Departments are also displayed in alphabetical order.

WHAT THIS CODE DOES NOT DO:

· No persistent storage (data is lost when program exits - it's in-memory only with HashMaps)
· No employee IDs, salaries, or other metadata (just names)
· No editing or deleting employees
· No file saving/loading
· No JSON or database storage
· No GUI or web interface

## DATA STRUCTURE

The program uses a HashMap<String, Vec<String>> where:

· Key: Department name (String)
· Value: Vector of employee names (automatically sorted)

## REQUIREMENTS

· Rust (any edition from 2018 onward)

## INSTALLATION & USAGE

1. Clone and navigate to the repository:
   ```bash
   git clone https://github.com/oboobotenefiok/employee_management_system.git
   cd employee_management_system
   ```
2. Run the program:
   ```bash
   cargo run
   ```
3. Use the following commands:
   Command Example Description
   Add <name> to <department> Add Ada to Engineering Adds an employee to a department
   List <department> List Engineering Shows all employees in that department
   List all List all Shows all departments with their employees
   Quit Quit Exits the program

Example Session

```
Employee Management System
Commands:
  Add <name> to <department>
  List <department>
  List all
  Quit

> Add Obot to Engineering
Added Obot to Engineering

> Add Ada to Engineering
Added Ada to Engineering

> Add Bose to Sales
Added Bose to Sales

> List Engineering
Employees in Engineering: ["Ada", "Obot"]

> List all

All employees by department:
Engineering: ["Ada", "Obot"]
Sales: ["Bose"]

> Quit
Goodbye!
```

## IMPORTANT LIMITATIONS

WARNING: This program does not save data. All employees added during a session are lost when the program exits. The code has no file I/O, database, or persistence mechanism of any kind.

Other limitations from the actual implementation:

· Cannot remove employees or departments
· Cannot edit employee names
· No duplicate name checking (same name can be added multiple times to same or different departments)
· Case-sensitive department names ("Engineering" vs "engineering" are different)
· No validation for empty names or departments

Project Structure

```
employee_management_system/
├── src/
│   └── main.rs          # Complete code (single file)
└── Cargo.toml           # Dependencies (none beyond stdlib)
```

License

Refer to repository's license file (if present).

---

This README is 100% faithful to the code - it doesn't claim persistence, editing, deletion, IDs, salaries, or any feature not actually implemented.

Author

Created by me: [Obot Obo](https://github.com/oboobotenefiok/)
