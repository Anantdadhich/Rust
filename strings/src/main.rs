fn main() {
    let mut s = String::new();

    let data = "intial";

    let s = data.to_string();
    let s = "intial".to_string();

    //    let s2 = "bar";
    //    s1.push_str(s2);
    //   println!("s2 is {s2}");

    //indexing in string
    let s1 = String::from("hello");
    //  let h = &s1[0];

    let s2 = String::from("tic");
    let s3 = String::from("tac");
    let s4 = String::from("toe");
    //At this point, s will be tic-tac-toe. With all of the + and " characters,
    // it’s difficult to see what’s going on. For combining strings in more complicated ways,
    // we can instead use the format! macro:
    let sa = s2 + "-" + &s3 + "-" + &s4;

    //  let ss = format!("{s2}-{s3}-{s4}");
}
