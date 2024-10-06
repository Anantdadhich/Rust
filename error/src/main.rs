use std::fs;

/*pub enum Option<T> {
    None,
    Some(T),
}
*/
///option enun in rust  
fn first_option(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    /*    let greetfile = fs::read_to_string("hel.txt");

    match greetfile {
        Ok(file_content) => {
            println!("file {}", file_content)
        }
        Err(error) => {
            println!("eror occur {}", error)
        }
    }*/

    ///for the option enum
    let mystring = String::from("radna");
    match first_option(mystring) {
        Some(index) => println!("the index of the string {}", index),
        None => println!(":theletter 'a' is not find "),
    }
}
