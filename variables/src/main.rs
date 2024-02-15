fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //Floating-Point Types
    let x = 2.5; // f64
    println!("The float value of x: {x}");

    let y: f32 = 3.8;
    println!("The float value of y: {y}");

    let sum = 5 + 10;
    println!("The value of sum: {sum}");

    let difference = 95.6 - 4.5;
    println!("The value of difference: {difference}");

    let product = 4 * 30;
    println!("The value of product: {product}");

    let quotient = 56.7 / 32.5;
    println!("The value of quotient: {quotient}");

    let truncted = -5  / 3; 
    println!("The value of truncted: {truncted}");

    let remainder = 43 % 5;
    println!("The value of remainder: {remainder}");

}