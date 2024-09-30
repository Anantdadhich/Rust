//A reference is like a pointer in that it’s an address we can follow to access the data stored at that address;
// that data is owned by some other variable. Unlike a pointer,
//a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

fn main() {
    //we create the string
    let s1 = String::from("hello");
    //we cal the len of string by function call
    let len = calculatelen(&s1); //it wiil  crete the reference
    println!("the leng of {s1} is {len}");
    //print the borrow
    let s2 = String::from("fakeer");
    borrow_fn(&s2);
    println!("the string length is {}", s2);
    //for the mutable string borrow
    //it will mutate the function it borrow
    let mut s3 = String::from("fakeer");
    change(&mut s3);
    //mutable reference
    let mut x1 = String::from("ji ha ");
    let x2 = &mut x1;
    //show error
    update_word(&mut x1);
    println!("{}", x1);
    println!("{}", x2);
}

fn calculatelen(s: &String) -> usize {
    s.len()
}

//for borrower
fn borrow_fn(some_string: &String) {
    println!("the {}", some_string); //some string is
}
//we wiil give the mutable references
fn change(string: &mut String) {
    string.push_str(",world")
}

fn update_word(word: &mut String) {
    word.push_str("main hu kahlanayak")
}
