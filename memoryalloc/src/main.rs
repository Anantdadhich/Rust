fn main() {
    call_stack();
    call_heap();
    call_update_fun();
}

fn call_stack() {
    let a = 2;
    let b = 3;
    let c = a + b;
    println!("the stack sum of {}and {} is  {} ", a, b, c);
}

fn call_heap() {
    let s1 = String::from("fids");
    let s2 = String::from("jutefn");

    let combined = format!("{} {}", s1, s2);
    println!("the combined {}", combined);
}
fn call_update_fun() {
    let mut s = String::from("before update  strig");
    println!("before update {}", s);

    s.push_str("and some add text");
    println!("after uodat {}", s)
}
