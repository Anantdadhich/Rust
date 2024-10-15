#[derive(Debug)] //it is used to print the ssturct filed

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //as we know self is refereence to current node
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//now we wirte the teest

#[cfg(test)]
mod tests {
    //this brings everything from outer scope
    use super::*;

    #[test]

    fn llargercanholdsmall() {
        let large = Rectangle {
            width: 8,
            height: 9,
        };

        let small = Rectangle {
            width: 4,
            height: 1,
        };

        assert!(large.can_hold(&small));
    }

    #[test]

    fn Smalleercanholdsmall() {
        let large = Rectangle {
            width: 8,
            height: 9,
        };

        let small = Rectangle {
            width: 4,
            height: 1,
        };

        assert!(!small.can_hold(&large));
    }
}
/*

pub fn greeting(naame: &str) -> String {
    format!("mainhu {name}!")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn greeting_namwe() {
        let result = greeting("dfdf");
        assert!(result.contains("dfdf"));
    }
}
*/

//#[should_panic]. The failure we got means that the code in the test function did not cause a panic.

pub struct Gueess {
    val: i32,
}

impl Gueess {
    pub fn new(val: i32) -> Gueess {
        if val < 1 || val > 100 {
            panic!("gueessn vvaluee mussr from 1 to 100 {val}");
        }
        Gueess { val }
    }
}

#[cfg(test)]

mod testss {
    use super::*;

    #[test]
    #[should_panic]
    fn greterthan() {
        Gueess::new(200);
    }
}
