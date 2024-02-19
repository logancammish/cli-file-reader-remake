use text_io::read;

fn main() {
    print!("Reading file...\r");
    print!("Is this file local (1) or on the web (2)?\r");
    let input: String = read!("{}\n");
    let input_: i32 = input.parse::<i32>().unwrap();
    println!("{input_}");
}
