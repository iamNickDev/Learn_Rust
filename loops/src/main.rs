fn main() {
    let mut counter = 0;
    let mut count = 0;
    let mut number = 3;
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // loop {
    //     println!("again!");
    //     break;
    // }

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF");

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value is: {element}");
    }

    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("Lets'go");
}
