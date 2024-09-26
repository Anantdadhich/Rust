//When a variable is immutable, once a value is bound to a name, you can’t change that value.

//You received the error message cannot assign twice to immutable variable `x`
// because you tried to assign a second value to the immutable x variable.

fn main() {
    let mut a = 5; //this is mutbale wwe can change the value beecaue we assign it by mut
                   //immutable means when we bind the vlaue we cannot change
    println!("the value of a is :{a}");
    a = 7;
    println!("the value of a is :{a}");
}

//let explore the booleans
/*
fn main() {
    let is_male = true;
    let is_above18 = true;

    if is_male {
        print!("you are male");
    } else {
        print!("you not")
    }

    if is_male && is_above18 {
        print!("you are legal")
    }
}

fn main() {
    let greet = String::from("hello");
    print!("{}", greet);
}

*/
