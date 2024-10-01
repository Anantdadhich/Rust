fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); //return the copy of the value

    let a = String::from("idfn df");

    println!("s1 {s1}  , s2 {s2}");

    takes_ownership(a);

    let x = 2;

    makescopy(x);

    //scoping varibales in same fn

    let d = 1;
    {
        let e = 3;
    }

    println!(""); //print the e
}

//function and ownership

fn takes_ownership(somestring: String) {
    println!("{somestring}");
}

fn makescopy(someint: i32) {
    println!("{someint}");
}
