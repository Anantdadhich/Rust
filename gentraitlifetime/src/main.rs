use std::fmt::Display;

fn main() {
    let string1 = String::from("the longet string");
    let string2 = "dfdf";

    let result = longest_with_annotation(string1.as_str(), string2, "today is sunday");

    println!("the striong res : {result}");
}
//, references must always be valid. If x or y were destroyed before the result is used, it could cause bugs. Lifetimes help avoid that.

//is a lifetime annotation, which tells the compiler that:
//The returned string slice will live as long as both x and y.
//Without this lifetime, the compiler wouldn't know how long the returned reference is valid.

fn longest_with_annotation<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("the annoouc {ann}");

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
