 use std::error::Error;
    use std::fs;

    pub struct Config {
        query:String,
        filepath:String
    }

    impl Config {
        pub fn build (args:&[String])->Result<Config , &'static str> {
            if args.len() <3 {
                return Err("no args");
            }

            let query=args[1].clone();
            let filepath=args[2].clone();

            Ok(Config {query,filepath})
        }
    }

    pub fn run(config:Config)->Result<(),Box<dyn Error>>{
      let contents=fs::read_to_string(config.filepath)?;
      println!("{contents}");
      Ok(())
    }