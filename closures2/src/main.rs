use std::thread;
use std::time::Duration;

fn generateworkout(inten:u32,raandom:u32){
 //doing closures
 let expenclossure=|num:u32|->u32{
    println!("cal slowly");
    thread::sleep(Duration::from_secs(2));
 };

 if inten <25 {
   println!("to {} pusshups",expenclossure(inten));
   println!("to {} situpss",expenclossure(inten));
 }else {
     if raandom ==3 {
        println!("take a break today");
     }else {
        println!("to {} minutes",expenclossure(inten)); 
     }
 }



}

fn main(){
    let simuluusserval=10;
    let simuranodom=7;
    generateworkout(simuluusserval,simuranodom);
}