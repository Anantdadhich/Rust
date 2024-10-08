//HashMap has keys of type String and values of type i32

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("dub"), 30);

    //they have special keyword like entry which defines the mutable reference
    scores.entry(String::from("Yellow")).or_insert(50);
    //accesing values in hasmap
    let teamname = String::from("rignef");
    let score = scores.get(&teamname).copied().unwrap_or(0);
    println!("the string {score}");
    //key value pair in hash map
    for (key, value) in &scores {
        println!("{key}:{value}");
    }

    //update the value based on hash
    let text = "tinefineifned";
    let mut mapp = HashMap::new();

    for word in text.split_whitespace() {
        let count = mapp.entry(word).or_insert(0);
        *count += 1;
    }
    println! {"{mapp:?}"};
}
