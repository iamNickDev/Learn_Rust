fn main() {
    let number = 3;
    let num = 6;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was somthing different then zero");
    }

    if num % 4 == 0 {
        println!("Number is divisible by 4");
    } else if num % 3 == 0 {
        println!("Number is divisible by 3");
    } else if num % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, 2");
    }

    let condition  = true;
    let number = if condition {5} else {6};
    println!("The value of number is : {number}");

 }
