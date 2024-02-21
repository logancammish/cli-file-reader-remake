use text_io::read;

fn main() {
    print!("Is this file local (1) or on the web (2)?\r\n\r");
    let input_: String = read!();
    let lw: i32 = input_.parse::<i32>().unwrap();
    print!("{lw} chosen...\r");

    print!("Please enter the relevant address or location of the file:\n\r\n");
    let location: String = read!(); 
    print!("{location} chosen...\r");
    
    print!("Reading file...\r");
}
