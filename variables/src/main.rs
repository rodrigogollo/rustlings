fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadowing();

    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    let t = true;
    let f: bool = false;

    let z = 'z';
    let heart_eyed_cat = '😻';

    let tup: (i8, i32, f64) = (127, 500, 3.0);
    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;


    // array -> unlikely to change
    // vector -> can change
    let a = [1, 2, 3, 4, 5];
    let first = a[0];


}

fn shadowing() {
    let x = 5;

    let x = x + 1; //6

    let x = x * 2; // 12
     
    println!("The value of x is: {}", x);
}
