//vittomazza's rust guessing game
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    loop {
        println!("Please input your guess.");

    let mut guess = String::new(); //crea una variabile mutabile (mut) che è bindata con una nuova stringa

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : u32 = match guess.trim().parse() {
        ok(num) => num, //if utente mette in input un valore compreso in u32 guess = num
        Err(_) => continue, //if utente mette in input qualsiasi cosa fuori da u32 ignora quell'input
    };

       println!("You guessed:{}", guess);

       println!("The secret number is {}", secret_number);
    
       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too low"),
           Ordering::Greater => println!("Too big"),
           Ordering::Equal => {
            println!("You won");
            break;
           }
      } 
    }
}