fn main() {
    //we have to find the index of first word of string
    let mut s = String::from("ji ha mainu hu");
    //lets create the string of refernce of the slice
    let ji = &s[0..2]; //we create the slice of the string
    let ha = &s[4..5];
    let mainu = &s[6..11]; //staring .. ending index in slcie

    let word = first_word(&s); //word will get the value

    println!("the word{word}");
    s.clear();
}
//take the reference of the string as variable
fn first_word(s: &String) -> &str {
    //unsigned integer
    let bytes = s.as_bytes(); //we convert our string to the arry of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
