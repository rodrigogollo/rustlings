use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let global_fps: u64 = 60;

    let width = 12;
    let height = 22;
    let mut board: Vec<Vec<String>> = vec![vec![String::from("  "); width]; height];

    for (i, row)  in board.iter_mut().enumerate() {
        for (j, item) in row.iter_mut().enumerate() {
            if i == 0 || j == 0 || i == 21 || j == 11 {
                *item = "[]".to_string();
            }
        }
    }

    game_loop(&board, global_fps);

}

fn game_loop(board: &Vec<Vec<String>>, fps: u64) {
    loop {
        thread::sleep(Duration::from_millis(fps));
        print_board(&board);
    }
}


fn main2(){
    let width = 12;
    let height = 22;
    let mut board: Vec<Vec<String>> = vec![vec![String::from("  "); width]; height];

    for (i, row)  in board.iter_mut().enumerate() {
        for (j, item) in row.iter_mut().enumerate() {
            if i == 0 || j == 0 || i == 21 || j == 11 {
                *item = "**".to_string();
            }
        }
    }

    print_board(&board);

    thread::sleep(Duration::from_secs(2));
    print_board(&board);

    let mut global_y = 8;

    for i in 1..board.len() {
        println!("{}:{}", i, board.len());
        if board[i][global_y] == "  ".to_string() {
            // line
            if i > 1 {
                board[i-1][global_y] = "  ".to_string();
            }
            // board[1 +i][7] = "  ".to_string();
            // board[1 +i][8] = "  ".to_string();
            // board[1 +i][9] = "  ".to_string();

            board[i][global_y] = "[]".to_string();
            // board[2 +i][7] = "[]".to_string();
            // board[2 +i][8] = "[]".to_string();
            // board[2 +i][9] = "[]".to_string();

            // triplet
            // board[1 +i][1] = "  ".to_string();
            // board[1 +i][2] = "  ".to_string();
            // board[1 +i][3] = "  ".to_string();
            // board[2 +i][2] = "  ".to_string();
            //
            // board[2 +i][1] = "[]".to_string();
            // board[2 +i][2] = "[]".to_string();
            // board[2 +i][3] = "[]".to_string();
            // board[3 +i][2] = "[]".to_string();
            //
            // cube
            // board[1 +i][11] = "  ".to_string();
            // board[1 +i][12] = "  ".to_string();
            // board[2 +i][11] = "  ".to_string();
            // board[2 +i][12] = "  ".to_string();
            //
            // board[2 +i][11] = "[]".to_string();
            // board[2 +i][12] = "[]".to_string();
            // board[3 +i][11] = "[]".to_string();
            // board[3 +i][12] = "[]".to_string();

            thread::sleep(Duration::from_millis(200));
            print_board(&board);
        }
    }
}

fn print_board(board: &Vec<Vec<String>>) {
    // Clear and move cursor top-left
    print!("\x1B[2J\x1B[1;1H"); 

    for (_i, row) in board.iter().enumerate() {
        for (_j, item) in row.iter().enumerate() {
            print!("{}", item);
        }
        println!();
    }

    // ensure the output is displayed immediately
    io::stdout().flush().unwrap();
}
//
// fn line() {
//     println!("[]");
//     println!("[]");
//     println!("[]");
//     println!("[]");
//
//     println!("");
// }
//
// fn cube() {
//     println!("[][]");
//     println!("[][]");
//
//     println!("");
// }
//
// fn triplet() {
//     println!("  []  ");
//     println!("[][][]");
//     println!("");
// }
