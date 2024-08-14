// use std::io;

fn main() {
    // loop {
    //     let mut line = String::new();
    //     io::stdin().read_line(&mut line)
    //         .expect("Failed to read line");
    //     println!("line {}", line);
    // }

    let a: [i32; 3] = [1, 2, 3];
    let curr: i32 = 0;    

    println!("curr {} arr {}", curr, a[curr as usize]);

    let curr_r = right(a, curr);
    println!("currR {} arr {}", curr_r, a[curr_r]);

    let curr_l = left(a, curr);
    println!("currL {} arr {}", curr_l, a[curr_l]);

    let current_left = move_left_right(a, curr, "left".to_string());
    println!("current_left arr {}", current_left);

    let current_right = move_left_right(a, curr, "right".to_string());
    println!("current_right arr {}", current_right);
}

fn right(arr: [i32; 3], curr: i32) -> usize {
    if (curr as usize) == arr.len() - 1{
        return 0 as usize;
    } 
    let size = curr + 1;
    return size as usize;
}

fn left(arr: [i32; 3], curr: i32) -> usize {
    if curr == 0 {
        return arr.len() - 1;
    } 
    let size = curr - 1;
    return size as usize;
}


fn move_left_right(arr: [i32; 3], curr: i32, action: String) -> i32 {
    let current;
    if action == "right" {
        current = right(arr, curr)
    }
    else if action == "left" {
        current = left(arr, curr)
    }
    else { 
        return arr[0];
    }
    return arr[current];

    // return match action {
    //     "right" => arr[right(arr, curr)],
    //     "left" => arr[left(arr, curr)]
    // };

}
