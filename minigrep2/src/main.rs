/*use std::env;
use std::fs;

fn main(){
  let args:Vec<String>=env::args().collect();

  let config=Config::new(&args);

  println!("Searching for {}",config.query);
  println!("file  {}",config.filepath);

  let contents=fs::read_to_string(config.filepath).expect("should have not read file");

  println!("with contents {contents}");


}

struct Config {
    query:String,
    filepath:String
}

impl Config {
    fn new (args:&[String])->Config{
        if args.len()< 3 {
            panic!("no enough");
        }

        let query=args[1].clone();
        let filepath=args[2].clone();

        Config{query,filepath}
    }
}
    */

use std::env;
use std::process;

use minigrep2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problemn   {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = minigrep2::run(Config) {
        println!("applica error {e}");
        process::exit(1);
    }
}
