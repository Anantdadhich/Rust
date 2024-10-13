// main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference

use std::result;

/*
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}



*/
fn main() {
    /*   let x = 5;

    let r = &x;
    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");  */

    let string1 = String::from("the string");
    // let result;
    {
        // let string2:String::from("the");
        //  result=longest(string1.as_str(), string2.as_str());
    }
    //  println!("the longest string {result}");
}
/*
fn longests(x:&str,y:&str)->&str{
  if x.len() > y.len(){
   x
  }else{
    y
  }
}

  */
//lifetime annotations
//to use it we define the generic lifetimes parameters
// 'a are lifetimes parametrs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//lifetimes annotations structure
struct ImportantExpect<'a> {
    part: &'a str,
}

fn mains() {
    let novel = String::from("string in ");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExpect {
        part: first_sentence,
    };
}

//lifetime ellision
