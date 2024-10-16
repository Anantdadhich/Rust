/*
fn main() {
    //we bring thrr use to usse the arguments function

    //in cases where the desired function is nested in more
    //than one module, we’ve chosen to bring the parent module into scope rather than the function

    let args: Vec<String> = env::args().collect();//We can use the collect function to create many kinds of collections, so we explicitly annotate the type of args to specify that we want a vector of strings
    dbg!(args);
}


*/
//this tells rust to use env modules

/*

use std::env; //contains
fn main() {
    let arguments: Vec<String> = env::args().collect(); //collect can coll the iterator in the vector
                                                        //we have to searh something
    let query = &arguments[1]; //we start the argument at one
                               //where the file might happen
    let filepath = &arguments[2];

    println!("search for {query}");
    println!("file {filepath}");
}

*/

/*

use std::env;
use std::fs; //it is used handle files

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[0];
    let filepath = &args[2];

    println!("the {query}");
    println!("the file {filepath}");
    //theey are used to for thee program logic
    let contents = fs::read_to_string(filepath).expect("should read file");

    println!("with {contents}");
}
*/
/*
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filepath) = parseconfigf(&args);

    println!("the {query}");
    println!("the file {filepath}");
    //theey are used to for thee program logic
    let contents = fs::read_to_string(filepath).expect("should read file");

    println!("the {contents}");
}
fn parseconfigf(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filepath = &args[2];

    (query, filepath)
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("the query");
    println!("the file ");
    //theey are used to for thee program logic
    let contents = fs::read_to_string(filepath).expect("should read file");

    println!("the {contents}");
}

struct Config {
    query: String,
    filepath: String,
}

fn parseconfigf(args: &[String]) -> Config {
    let query = args[1].clone();
    let filepath = args[2].clone();

    Config { query, filepath }
}
