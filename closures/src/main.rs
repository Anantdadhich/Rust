#[derive(Debug, PartialEq, Clone, Copy)]

enum Shirtcolor {
    Blue,
    Red,
}

struct ShirtEnv {
    shirts: Vec<Shirtcolor>,
}
// we define nthe function for the sturctures
impl ShirtEnv {
    //now we will choose which tshirt we will givve to the user

    //option means we will provide the user or might it would be color of the tshirt
    fn giveaway(&self, userprefer: Option<Shirtcolor>) -> Shirtcolor {
        //if we give thee preferce to the user will choos the color

        //f the userprefer is Some(color), it returns that color.
        //If the userprefer is None, it calls moststock to find the most stocked color.

        userprefer.unwrap_or_else(|| self.moststock())
    }
    //stock tshirt
    fn moststock(&self) -> Shirtcolor {
        let mut numblue = 0;
        let mut numred = 0;

        for color in &self.shirts {
            match color {
                Shirtcolor::Blue => numblue += 1,
                Shirtcolor::Red => numred += 1,
            }
        }

        if numred > numblue {
            Shirtcolor::Red
        } else {
            Shirtcolor::Blue
        }
    }
}

fn main() {
    let store = ShirtEnv {
        shirts: vec![Shirtcolor::Blue, Shirtcolor::Red, Shirtcolor::Blue],
    };

    let userprefer = Some(Shirtcolor::Red);

    let giveawa = store.giveaway(userprefer);

    println!("the user w9i9th {:?} is {:?}", userprefer, giveawa);

    let userpreef2 = None;
    let giveawa2 = store.giveaway(userpreef2);

    println!("the second usser {:?} with {:?}", userpreef2, giveawa2);
}
