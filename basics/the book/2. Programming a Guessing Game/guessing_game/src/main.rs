use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  // Prints a message to the console
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  // Prints a message to the console
  println!("Please input your guess.");

  loop {
    // Creates a mutable string variable to store the user's guess
    /*
      The :: syntax in the ::new line indicates that new is an associated function of the String type.
      An associated function is a function that’s implemented on a type, in this case String. 
      This new function creates a new, empty string. You’ll find a new function on many types because it’s 
      a common name for a function that makes a new value of some kind.

      In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound 
      to a new, empty instance of a String.
    */
    let mut guess = String::new();

    // Reads a line from the standard input and stores it in the `guess` variable
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    // Prints the user's guess
    println!("Your guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;

      }
    }
  }

  
}
