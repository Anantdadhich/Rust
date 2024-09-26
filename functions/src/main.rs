//we just write the for even or odd
pub fn main() {
    let x = 99;
    let iseven = is_even(x);

    if iseven {
        print!("{}is even", x);
    } else {
        print!("{}is odd", x)
    }
    statements();

    loops();

    counter();
}
// we jsut give parameter
pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

//for statements
fn statements() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!(" the value is :{y}");
}

//loops
pub fn loops() {
    let str = String::from("hello");
    println!("First name {}", get_first_name(str));
}

pub fn get_first_name(str: String) -> String {
    let mut firstname = String::from("");

    for c in str.chars() {
        if c == ' ' {
            break;
        }
        firstname.push(c);
    }
    return firstname;
}

pub fn counter() {
    let mut counter = 0;
    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is :{result}");
}
