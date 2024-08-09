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

    let result = sum(cube, 3, 5);
    println!("res: {}", result);
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
