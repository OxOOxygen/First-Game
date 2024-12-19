use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    
    let secret=rand::thread_rng().gen_range(1..=1000);
    loop{
        let mut guess=String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to load");
    println!("Guess the number ");

let guess:i32=match guess.trim().parse(){
    Ok(num)=>num,
    Err(_)=>continue,

};
match guess.cmp(&secret) {
    Ordering::Equal=> {
        print!("win");
        break;
    }
    Ordering::Greater=>println!("number is greater"),
    Ordering::Less=>println!("number is smaller")
    
}




        }

}
