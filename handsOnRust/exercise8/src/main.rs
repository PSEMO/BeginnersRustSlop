use std::collections::HashMap;

#[derive(Debug)]
enum EmployeeStatus {
    Active,
    OnLeave,
    Resigned,
}

struct Employee {
    name: String,
    status: EmployeeStatus,
}

fn main() {
    //****************
    let mut storage: Vec<Employee> = Vec::new();
    
    storage.push(
        Employee {name: String::from("mahmut"), status: EmployeeStatus::Active});
    storage.push(
        Employee {name: String::from("kemal"), status: EmployeeStatus::OnLeave});
    storage.push(
        Employee {name: String::from("memet"), status: EmployeeStatus::Resigned});
    
    for employee in &storage {
        printEmployee(employee);
    }
    
    println!("------------------");
    //****************

    //****************
    println!("{}", getSecondWord(String::from("Add Alice to Engineering")));
    println!("{}", getLastWord(String::from("Add Alice to Engineering")));
    println!("{}", getSecondWord(String::from("Add Bob to Sales")));
    println!("{}", getLastWord(String::from("Add Bob to Sales")));
    
    println!("------------------");
    //****************
    
    //****************
    let mut department = HashMap::new();

    department.insert(String::from("Accounting"), storage);

    let mut storage: Vec<Employee> = Vec::new();
    
    storage.push(
        Employee {name: String::from("Shinji"), status: EmployeeStatus::Active});
    storage.push(
        Employee {name: String::from("Asuka"), status: EmployeeStatus::OnLeave});
    storage.push(
        Employee {name: String::from("Mei"), status: EmployeeStatus::Resigned});

    department.insert(String::from("Pilots"), storage);

    for (key, value) in department {
        println!("{}", key);
        for employee in &value {
            printEmployee(employee);
        }
        println!("******");
    }

    println!("------------------");
    //****************
}

fn printEmployee(employee: &Employee) {
    println!("{0}, {1:?}", employee.name, employee.status);
}

fn getSecondWord(s: String) -> String {
    let mut i = 0;
    for word in s.split_whitespace() {
        if i == 1 {
            return word.to_string();
        }
        i += 1;
    };

    return String::from("N/A");
}

fn getLastWord(s: String) -> String {
    let mut lastWord: String = String::from("N/A");
    
    for word in s.split_whitespace() {
        lastWord = word.to_string();
    };

    return lastWord;
}