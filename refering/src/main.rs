//we define the modules

mod backofhome {
    pub struct Breakfast {
        pub toast: String,
        seasonlfood: String,
    }
    //struct implementatin
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonlfood: String::from("pocdds"),
            }
        }
    }
}

pub fn main() {
    let mut meal = backofhome::Breakfast::summer("thyue f");

    meal.toast = String::from("whar");

    print!("the meal {}", meal.toast);
}
