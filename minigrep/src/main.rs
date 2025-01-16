/* Clone simplficado do comando grep existente em Unix */

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    
    println!("In file {file_path}");

    let content = fs::read_to_string(file_path).expect("Cannot open the file");

    println!("With text:\n{content}");
}