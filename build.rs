/*
 * @Author: plucky
 * @Date: 2022-10-10 11:51:28
 * @LastEditTime: 2023-06-27 07:22:07
 * @Description: 
 */

use std::process::Command;

fn main(){
    // tailwindcss -i index.css -o style.css
    Command::new("tailwindcss").args(["-i", "index.css", "-o", "dist/.stage/style.css"])
        .status().unwrap();
    // cp static/favicon.ico ... dist/.stage
    // Command::new("cp").args(&["static/favicon.ico", "...", "dist/.stage/"])
    // .status().unwrap();
}