fn main() {

    //Ownership rules
    // 1. Each value in Rust has a variable called its owner.
    // 2. There can only be one owner at a time (a variable cannot have two owners at the same time)
    // 3. When the owner foes out of scope, the value will be dropped



    {
        // x is not valid here, it's not yet declared
        let x = "Hello"; // x is valid from this point forward and is hard coded directly into the final executable
        println!("x is {}", x);
    } // this scope is now over and x is no longer valid

        {
        // s is not valid here, it's not yet declared
        let s: String = String::from("Hello"); // s is valid from this point forward, s is allocated on heap
        println!("x is {}", s);
    } // this scope is now over and s is no longer valid

}