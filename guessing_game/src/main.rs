use std::io;  //From the standard library, we are bringing the io library into scope
//By Default, there is a few items in the standard library that rust automatically brings into scope for every program, this is called prelude






fn main() {
    println!("Guess the number!");
    //println! calls a Rust macro. If it called a function instead, it would be entered as println (without the !). 
    //For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function, 
    //and that macros don’t always follow the same rules as functions.

    println!("Please input your guess.");

    //In Rust, variables and references are immutable by default.
    //To make the variable mutable, we used the mut signifier before the variable name
    let mut guess = std::string::String::new(); // RHS, is a function that returns a new instance of a String which is heap allocated and growable. This is an example of a prelude function.
    // '::' signifies in the above case that new is an associated function of the string type

    
    //thanks to the import of the io library from the std module, we can call the function stdin which allow us to handle input
    //The stdin function returns an instance of std::io::Stdin, 
    //which is a type that represents a handle to the standard input for your terminal.
    io::stdin()
        .read_line(&mut guess)// calls the read_line method from Stdin, which appends input to a string without overwriting thus need mutable string
        .expect("Failed to read line"); //read_line also returns value, io::Result allows for error handling, expect is a method of io:Result

    println!("You guessed: {}", guess);
}

//Ended at Testing the First Part of Guessing game