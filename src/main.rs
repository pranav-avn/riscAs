mod parse;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io;

fn read_file(filename: &str) -> Result<BufReader<File>, io::Error>{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let asm_file_path = &args[1];
    let output_file_name = &args[2];
    let ext:&String = &".bin".to_string();
    let output_file = format!("{}{}",output_file_name, ext);
    match read_file(&asm_file_path){
        Ok(contents) => parse::asm_parser(contents,output_file),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
