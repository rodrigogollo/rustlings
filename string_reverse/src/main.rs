use std::io;

fn main() {

    println!("Input a word:");

    let mut word = String::new();

    io::stdin().read_line(&mut word).expect("Failed to read line");

    // let word = match word.trim() {
    //     Ok(word) => word,
    //     Err(_) => println!("error"),
    // };
    
    let word = word.trim();
    
    let char_vec: Vec<char> = word.chars().collect();

    let mut reversed = String::new();

    for (_i, c) in char_vec.iter().enumerate().rev() {
        // println!("{} {}", i, c);
        reversed.push(*c);
    }
    

    println!("You wrote: {}", word);
    println!("Reversed: {}", reversed);

}
