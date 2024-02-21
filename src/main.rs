use std::{fs::File, io::{self, BufRead}, path::Path, env, process::exit, thread, time};
use text_io::read;

struct Program { }
impl Program { 
    fn sleep(t: u64) { 
        thread::sleep(time::Duration::from_millis(t));
    }
    fn read_lines(name: String) -> io::Result<io::Lines<io::BufReader<File>>> { 
        let file = File::open(name)?;
        return Ok(io::BufReader::new(file).lines());
    }
}
fn main() { 
    let mut args: Vec<_> = env::args().collect();

    if !(args.len()>1) {
        // terminal arguments      
        args = Vec::from(
            ["".to_string(), "".to_string(), "".to_string()]);

        print!("Is this file local (1) or on the web (2)? ");
        args[1] = read!();
        Program::sleep(1000);

        print!("Please enter the relevant address or location of the file: ");
        args[2] = read!(); 
        Program::sleep(1000);
        
        print!("How long do you want to wait between lines (ms)? ");
        args[3] = read!(); 
        Program::sleep(1000);
    } else if !(args.len()==4) {
        println!("\n\n!!! Invalid number of arguments !!! \n\n");
    }

    let local = args[1].parse::<i32>().unwrap(); 
    let speed = args[3].parse::<u64>().unwrap();

    if local == 1 {
        println!("Reading file...\n");
        if let Ok(lines) = Program::read_lines(format!("{}", args[2])) {
            for line in lines.flatten() {
                Program::sleep(speed);
                println!("{line}");
            }
        } else { 
            println!("File not found (did you spell it correctly?)");
        }
    } else if local == 2 { 
        println!("Not implemented yet");
    } else {
        println!("Invalid input");
        exit(2);
    }
}
