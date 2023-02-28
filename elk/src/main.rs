use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: elk FILE");
    let input = fs::read(&input_path)?;
    if let Ok(result) = delf::File::parse(&input[..]).map_err(|e| format!("{:?}", e)) {
        let (_, file) = result;
        println!("input is a supported ELF file!");
        println!("{:?}", file);
    }

    Ok(())
}
