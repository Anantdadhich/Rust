use std::fmt::Display;

fn main() {
    let string1 = String::from("the longet string");
    let string2 = "dfdf";

    let result = longest_with_annotation(string1.as_str(), string2, "today is sunday");

    println!("the striong res : {result}");
}

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
