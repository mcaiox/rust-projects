fn main() {

    let mut s = String::from("hello");


    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    change(&mut s);


    println!("The length of '{}' is {}.", s, len);
    
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Rules of References
// - At any given time you can have either one mutable reference or any number of immutable references
// - References must always be valid