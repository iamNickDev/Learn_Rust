fn main() {
    // let y = 6; is a statement.
    let y = {
        let n = 6;
        n + 4
    };

    println!("The value of y is: {y}");

    println!("Hello, world!");

    // another_function();
    another_function(5);
    print_labeled_measurement(10, 'h');

    let r = five();
    println!("The value of r is: {r}");

    let p = plus_one(18);
    println!("The value of p is: {p}");
}

fn another_function(x: i32) {
    // println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_lable: char) {
    println!("The Measurement is: {value}{unit_lable}");
}

fn five() -> i32 {
    return 5;
}

fn plus_one(p: i32) -> i32 {
    p + 1
}
