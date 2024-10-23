/*use std::thread;
use std::time::Duration;

fn generateworkout(inten: u32, raandom: u32) {
    //doing closures  |parameteers| expression
    //we use closurre beacussee it captures logic for a repeated task
    //now we not the function multiple tiimes
    let expenclossure = |num: u32| -> u32 {
        println!("cal slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if inten < 25 {
        println!("to {} pusshups", expenclossure(inten));
        println!("to {} situpss", expenclossure(inten));
    } else {
        if raandom == 3 {
            println!("take a break today");
        } else {
            println!("to {} minutes", expenclossure(inten));
        }
    }
}

fn main() {
    let simuluusserval = 10;
    let simuranodom = 7;
    generateworkout(simuluusserval, simuranodom);
}
*/
/* fn main(){
    let list=vec![1,2,3];

    println!("before borrow {list:?}");
    let onlyborrow= || println!("the clousre");
        println!("before borrow {list:?}");
        onlyborrow();
            println!("after  borrow {list:?}");
}*/

/*
fn main(){
    let list=vec![1,2,3];

    println!("before borrow {list:?}");
    let onlyborrow= || println!("the clousre");
        println!("before borrow {list:?}");
        onlyborrow();
            println!("after  borrow {list:?}");
}



*/

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("before {list:?}");

    //We spawn a new thread, giving the thread a closure to run as an argument.

    thread
        .spawn(move || println!("the thread {list:?}"))
        .join()
        .unwrap();
}
