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

    let x = 5;
    let y = x;
    // Both vars equal 5 and since integers are simple values with a known, fixed size these two 5 values are pushed onto the stack
        println!("x is {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    // Here however, the same rules do not apply 
    // A string is made up on three parts, so s1 will:
    // 1. have a ptr that points to the memory that holds the contents of the string - ['h','e','l','l','o']
    // 2. a length - 5
    // 3. a capacity - 5
    // This group of data is stored on the stack whereas the contents of the array that it points to is on the heap.

    // When s1 is assigned to s2, the string data(ptr, len and capacity) listed above is copied to the stack, we don't however copy the data on the heap the ptr refers to
    // When rust wants to deallocate when s1 and s2 go out of scope it will free up memory but if there are two owners, then its an issue (double free error)
    // Rust deals with this by considering s1 no longer valid after "let s2 = s1;", thus if you try access s1, it wont work.
    
 
}