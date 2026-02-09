use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guesss the nunmber");
    let secret_num = rand::thread_rng().gen_range(1..=100);
  loop{
    println!("Please enter your number ");
    let mut guess = String::new();
    io::stdin().read_line( &mut guess).expect("failed to read user input");

    let  guess:u32 = guess.trim().parse().expect("failde to parese");
    match guess.cmp(&secret_num){
    Ordering::Equal=>{
        println!("you won");
    break;
}

    Ordering::Greater=>println!("Too big"),
    Ordering::Less=>println!("Too Small"),

}
}
}
