use std::io;

fn main() {
    println!("Guessing the Number!");

    println!("Please Input the Number");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");


    println!("You gussed {guess}")

}