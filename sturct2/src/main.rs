//we use the struct why
//area od rect
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("the react {}", area(&rect1));

    let squ1 = Square {
        widht: 40,
        height: 40,
    };

    println!("the area of sq {}", squ1.ar());
}

fn area(recta: &Rectangle) -> u32 {
    recta.height * recta.width
}

//method syntax

//Methods are similar to functions: we declare them with the fn keyword and a name,
//they can have parameters and a return value, and
// they contain some code that’s run when the method is called from somewhere else.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object

struct Square {
    widht: u32,
    height: u32,
}

impl Square {
    fn ar(&self) -> u32 {
        self.height * self.widht
    }
}
