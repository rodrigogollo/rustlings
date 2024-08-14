fn main() {
    let number = 3;

    if number > 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let x = if number > 5 {
        6
    } else {
        4
    };

    println!("the value of x is: {}", x);
}
