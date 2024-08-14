fn main() {
    println!("Hello, world!");
    // let a = [1, 2, 3];
    // for element in a.iter() {
    //     println!("{}", element);
    // }
    //
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }

    let f = 250.0;
    let c = farenheit_to_celcius(f);
    println!("{} Farenheit is equal to {} Celcius", f, c);
}

fn farenheit_to_celcius(x:f64) -> f64 {
   (x - 32.0) * 5.0/9.0
}
