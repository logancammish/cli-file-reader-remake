use std::{thread, time};

use text_io::read;

fn main() {
    print!("Is this file local (1) or on the web (2)?\r\n\r");
    let input_: String = read!();
    let lw: i32 = input_.parse::<i32>().unwrap();
    thread::sleep(time::Duration::from_millis(1000));
    print!("{lw} chosen...\r");
    thread::sleep(time::Duration::from_millis(1000));

    print!("Please enter the relevant address or location of the file:\n\r\n");
    let location: String = read!(); 
    thread::sleep(time::Duration::from_millis(1000));
    print!("{location} chosen...\r");
    thread::sleep(time::Duration::from_millis(1000));
    
    print!("Reading file...\r");
}
