fn main() {
    //we define the vector
    let numberlist = vec![34, 56, 56, 35, 84];

    let mut largest = &numberlist[0];

    for number in &numberlist {
        if number > largest {
            largest = number;
        }
    }

    println!("the largret  {largest}");
}

//how to use geneics or list
/*
fn largeest<T>(list: &[T]) -> &T {
    let mut lar = &list[0];
    /*
    for item in list {
        if item < lar {
            lar = item;
        }
    }  */
}
*/

//generics in  struct

struct generic<T> {
    s: T,
    d: T,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
//t is function whnat type of it when you used it

//struct can hold
//but generics can hold any thing
struct Point<T> {
    a: T,
    b: T,
}
/*
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.a;
    }
}
    */

//create two points

struct Points<X1, Y1> {
    x: X1,
    y: Y1,
}
/*
impl<X1, Y1> Points<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Points<X2, Y2>) -> Points<X1, Y1> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}
*/
