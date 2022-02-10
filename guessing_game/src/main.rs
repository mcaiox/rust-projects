use std::io;  //From the standard library, we are bringing the io library into scope
//By Default, there is a few items in the standard library that rust automatically brings into scope for every program, this is called prelude
use rand::Rng; // The Rng trait defines methods that random number generators implement, this trait must be in scope to use those methods. 
use std::cmp::Ordering;
 



fn main() {
    println!("Guess the number!");
    //println! calls a Rust macro. If it called a function instead, it would be entered as println (without the !). 
    //For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function, 
    //and that macros don’t always follow the same rules as functions.
    let secret_number = rand::thread_rng().gen_range(1..101);
    //the rand::thread_rng function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system

    loop{
        println!("Please input your guess.");

        //In Rust, variables and references are immutable by default.
        //To make the variable mutable, we used the mut signifier before the variable name
        let mut guess = String::new(); // RHS, is a function that returns a new instance of a String which is heap allocated and growable. This is an example of a prelude function.
        // '::' signifies in the above case that new is an associated function of the string type

        
        //thanks to the import of the io library from the std module, we can call the function stdin which allow us to handle input
        //The stdin function returns an instance of std::io::Stdin, 
        //which is a type that represents a handle to the standard input for your terminal.
        io::stdin()
            .read_line(&mut guess)// calls the read_line method from Stdin, which appends input to a string without overwriting thus need mutable string
            .expect("Failed to read line"); //read_line also returns value, io::Result allows for error handling, expect is a method of io:Result
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){ // Cmp method compares two values and returns a variant of the Ordering enum
            //we use match expression to decide what to do next base on which variant of Ordering was returned
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

//Ended just before the section titled "Comparing the Guess to the Secret Number"