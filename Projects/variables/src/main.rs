fn main() {
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}")
}