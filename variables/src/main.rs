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

    let truncted = -5 / 3;
    println!("The value of truncted: {truncted}");

    let remainder = 43 % 5;
    println!("The value of remainder: {remainder}");

    let t = true;
    println!("boolean: {t}");

    let f: bool = false;
    println!("boolean status: {f}");

    let c = 'z';
    println!("character1: {c}");

    let z: char = 'ℤ';
    println!("character2: {z}");

    let heart_eyed_cat = '😻';
    println!("character3: {heart_eyed_cat}");

    let tup = (-15, 2.5, 500);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {y}");

    let new_tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = new_tup.0;
    println!("The value of five_hundred is:{five_hundred}");

    let six_point_four = new_tup.1;
    println!("The value of six_point_four is:{six_point_four}");

    let one = new_tup.2;
    println!("The value of one is:{one}");

    let _a = [1, 2, 3, 4, 5];
    // println!("The value of a is:{a}");

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let feb = _months[11];
    println!("{}", feb);
}
