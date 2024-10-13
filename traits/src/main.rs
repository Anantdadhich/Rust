//In Rust, traits are like a set of rules that tell a type (like a struct or enum) what behaviors (or methods) it must have.
//Think of traits as promises that a type will do certain things

//i define the trait
pub trait Summary {
    fn sumarize(&self) -> String {
        // // A method that types must implement
        return String::from("h i");
    }
}
//now we define thne structure
struct User {
    name: String,
    id: u32,
}

impl Summary for User {
    fn sumarize(&self) -> String {
        return format!("the user {} is {} ", self.name, self.id);
    }
}

//trait as an  argument
pub fn notfiy(item: &impl Summary) {
    println!("the argument vlaue is{}", item.sumarize());
}

//how we create trait bound syntax
//the generic is bound to the trait
pub fn notify<T: Summary>(item: T) {
    println!("the trait is {}", item.sumarize());
}
//we can bound to multiple traits

/*
pub fn notify<T: Summary + Fix>(item: T) {
    println!("the trait is {}", item.sumarize());
}*/

fn main() {
    let user = User {
        name: String::from("nudf"),
        id: 23,
    };
    println!("the summ {}", user.sumarize());
}
