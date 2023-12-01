use std::{fs::File, io::Read};

fn main() {
    let mut file_handle = File::open("sample.input").unwrap();

    let mut sample_data = String::new();
    file_handle.read_to_string(&mut sample_data).unwrap();

    let mut total: u64 = 0;

    for row in sample_data.split("\n") {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for char in row.chars() {
            if char.is_numeric() {
                if first.is_none() {
                    first = Some(char);
                }

                last = Some(char);
            }
        }

        if first.is_none() || last.is_none() {
            continue;
        }
        let addative: u64 = format!("{}{}", first.unwrap(), last.unwrap()).parse().unwrap();

        total += addative;
    }

    println!("{}", total);
}
