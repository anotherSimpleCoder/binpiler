use std::{fs::File, io::{self, BufRead, Write}};
use regex::Regex;

/*
    Reads file and returns it strings
 */
pub fn read_file(path_string: String) -> io::Result<Vec<String>> {
    //Open file
    let file = File::open(path_string);

    match file {
        Ok(_) => {},
        Err(x) => panic!("{}", x)
    }

    //Read the file and check for valid 8bit binary numbers and add them to a vector
    let file_pointer = file.unwrap();
    let reader = io::BufReader::new(file_pointer);

    let mut string_vector: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(x) => {
                //Check for valid length
                if x.len() > 8 {
                    panic!("Invalid binary size");
                }

                //Check for valid format
                let pattern = Regex::new(r"[^01]+").unwrap();

                if let Some(_) = pattern.find(&x) {
                    panic!("Invalid binary!");
                }

                string_vector.push(x);
            },
            Err(e) => panic!("{}", e)
        }
    }

    Ok(string_vector)
}

//Eval numbers and return a list of binary numbers
pub fn eval_numbers(numbers: Vec<String>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for i in numbers {
        let bin = u8::from_str_radix(i.as_str(), 2).unwrap();
        res.push(bin);
    }

    return res;
}

pub fn write_file(path: String, binaries: Vec<u8>) -> io::Result<()>{
    let file = File::create(path);
    let mut file_pointer: File;

    match file {
        Ok(x)=> {file_pointer = x},
        Err(e)=> panic!("{}", e)
    }

    file_pointer.write_all(&binaries.as_slice())?;

    Ok(())
}