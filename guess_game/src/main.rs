use std::io; //To obtain user input and then print the result as output -- io == input output
use std::cmp::Ordering; // still wondering how toget familiar with all these inbuilt modules
use rand::Rng;
fn main() {
    println!("Guess the number!");
    //loop can be done with loop keyword
    loop {
        println!("Please input your guess.");

        //let apples = 5; // immutable
        //let mut bananas = 5; // mutable
        let _secret_number = rand::thread_rng().gen_range(1..=100); // using _ makes unused var private just womndering

        let mut guess = String::new(); // create variable --- String::new, a function that returns a new instance of a String
        println!("{}", _secret_number); // seems I can't call like in python print(_secret_number) rather must use println!("{}",_secret_number) . why must it be "{}", variable??
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess: u32 = guess.trim().parse().expect("Please type numbers"); // make sure to use ;
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
