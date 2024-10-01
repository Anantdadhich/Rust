//we define the struct after it we defnine the instance of the stur

//now we build the user function to return it

struct User {
    email: String,
    username: String,
    active: bool,
}
#[derive(Clone, Copy)]
struct Stud {
    active: bool,
    sigin_count: u64,
}

//unlike strucut
struct Always; //we does not require the currly braq

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
    }
}

fn main() {
    //.to define the instance we startr with same name
    let mut user1 = User {
        email: String::from("ana@gmail.com"),
        username: String::from("sjd"),
        active: true,
    };
    //to get sepcific val from struct do dot notation

    //  user1.email = String::from("anoana@gmail.com");

    let user2 = build_user(String::from("dfodfsdsd"), String::from("eifnf"));

    println!("the suer{}", user2.email);

    //for the copy.=,clon e
    let mut stud1 = Stud {
        active: true,
        sigin_count: 1,
    };

    print_name(stud1);
    print!("stud  username {}", stud1.active);

    let sub = Always;
}

fn print_name(stud1: Stud) {
    print!("stud  username {}", stud1.active);
}
