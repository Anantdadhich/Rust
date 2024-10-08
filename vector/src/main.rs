fn main() {
    //conatin the arguments
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("the element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the element is {third}"),
        None => println!("no element "),
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
