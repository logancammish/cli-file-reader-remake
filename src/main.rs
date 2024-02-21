use std::{env, thread, time};
use text_io::read;

struct Program { }
impl Program { 
    fn sleep(t: u64) { 
        thread::sleep(time::Duration::from_millis(t));
    }
}
fn main() { 
    let mut args: Vec<_> = env::args().collect();

    if !args.len()>1 {
        // terminal arguments      
        args = Vec::from(
            ["".to_string(), "".to_string(), "".to_string()]);

        print!("Is this file local (1) or on the web (2)? ");
        args[1] = read!();
        Program::sleep(1000);

        print!("Please enter the relevant address or location of the file: ");
        args[2] = read!(); 
        Program::sleep(1000);
    }
    println!("Reading file...\n");
}
