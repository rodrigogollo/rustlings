use std::io;

fn main() {
    // buzz(5);
    // buzz(7);
    // buzz(17);
    // loop {
    //     println!("Please input a number.");
    //
    //     let mut guess = String::new();
    //
    //     io::stdin().read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => 0,
    //     };
    //
    //     let result = abs(guess);
    //     println!("Result: {}", result);
    // }

    // let result = sum(cube, 3, 5);

    let x = 6;
    let result = factorial(x);
    let result2 = factorial2(x);
    let result3 = factorial3(1, 1, x);
    println!("res: {}, {}, {}", result, result2, result3);
}

fn abs(x: i32) -> i32 {
    if x < 0 {
        return x * -1;
    } else { 
        return x;
    }
}

fn buzz(x: i32) {
    if x % 7 == 0 {
        println!("Buzz");
    } else {
        println!("{}", x);
    }
}

fn sum(func: fn(i32) -> i32, x: i32, y: i32) -> i32 {
    if x > y {
        return 0;
    }
    else {
        return func(x) + sum(func, x + 1, y);
    }
}

fn square(x: i32) -> i32 {
   return x * x; 
}

fn cube(x: i32) -> i32 {
    return x * x * x;
}

fn factorial(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }
    return x * factorial(x-1);
}

fn factorial2(max: i32) -> i32 {
    let mut counter = 1;
    let mut result = 1;
    while counter <= max {
        result = result * counter; 
        counter = counter + 1;
    }
    return result;
}

fn factorial3(x: i32, result: i32, max: i32) -> i32 {
    if x > max {
        return result;
    }
    return factorial3(x+1, result * x, max);
}
