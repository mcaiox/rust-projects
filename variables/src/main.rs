fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    // x = 6;
    // println!("The value of x is: {}", x);
    // println!("The value of THIS is: {}", THREE_HOURS_IN_SECONDS );
    // let tup:(i32, f64, u8) = (500, 6.4, 1);
    // let (_xx, y, _z) = tup;
    let x = five();

    println!("five() is: {} ",x);

}

fn five() -> i32 {
    5
}
