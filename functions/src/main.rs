fn main() {
    println!("Hello, world!");

    another_function(5);
    let x = five();
    println!("the value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("the value of x is: {}", x);
}


fn expressions() {
    // let x = (let x = 5); // error let inside () is not an expression

    let x = {
        let x = 3;
        x + 1
    };
}

fn five() -> i32 {
    5 
}

fn compose(f, g){ 
    return (fn (x) { return f(g(x)) })
}
