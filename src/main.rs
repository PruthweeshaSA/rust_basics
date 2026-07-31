use std::fs;
use std::io;

fn readfile(filename: String) -> io::Result<String> {
    fs::read_to_string(filename)
}

fn main() -> io::Result<()> {
    println!("Hello, please select the file you wish to view:");
    for entry in fs::read_dir("./src")? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    
    let mut input_filename = String::new();
    io::stdin().read_line(&mut input_filename)?;
    let cleaned_input = input_filename.trim();
    let content = readfile(cleaned_input.to_string())?;
    println!("content:");
    println!("{content}");
    Ok(())
}
