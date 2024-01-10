use std::{fs, env};

fn main() {

    if env::args().len() <= 1 {
        println!("Program requires at least 1 argument.");
        return;
    }

    let name_to_check: String = env::args().nth(1).unwrap();

    let path_to_check: &str = "class.txt";

    let class_names: String = fs::read_to_string(path_to_check).unwrap();

    let mut found_name: bool = false;
    for line in class_names.lines() {
        if line == name_to_check {
            found_name = true;
            break;
        }
    }
    if found_name {
        println!("Name was found in txt file!");
    } else {
        println!("Sadly the name wasn't found in the txt file :'(");
    }

}

