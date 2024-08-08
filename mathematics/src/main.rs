use std::io;

fn main() {
    buzz(5);
    buzz(7);
    buzz(17);
    loop {
        println!("Please input a number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        let result = abs(guess);
        println!("Result: {}", result);
    }
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
