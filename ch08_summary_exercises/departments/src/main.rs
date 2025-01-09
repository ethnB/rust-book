use std::{collections::HashMap, io};

fn main() {
    let mut company = HashMap::new();

    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    let mut instruction = String::new();
    while instruction != "exit" {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading user input");

        let input = input.trim().to_lowercase();

        let mut words = input.split_whitespace();

        instruction = words.next().unwrap_or("").to_string();

        match instruction.as_str() {
            "add" => {
                let name = words.next().unwrap().to_string();
                // skip
                words.next().unwrap();
                let department = words.next().unwrap().to_string();

                let people = company.entry(department).or_insert(vec![]);
                people.push(name);
            }
            "list" => {
                let department = match words.next() {
                    Some(v) => v.to_string(),
                    None => String::from(""),
                };

                let mut departments;
                if department == "" {
                    departments = company.keys().map(|k| k.to_string()).collect();
                } else {
                    let mut v = Vec::new();
                    v.push(department);
                    departments = v;
                }

                departments.sort();

                for d in departments {
                    match company.get(&d) {
                        Some(people) => {
                            println!("Department: {} People:", d);
                            for p in people {
                                println!("{}", p);
                            }
                        }
                        None => println!("Department {} not found!", d),
                    }
                }
            }
            _ => (),
        };
    }
}
