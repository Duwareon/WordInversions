use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn getlist(file: File) -> std::io::Result<Vec<String>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    
    let filesplit = contents.split("\r\n");

    let mut words = Vec::new();

    for i in filesplit {
        words.append(&mut vec![i.to_string()]);
    }

    Ok(words)
}

fn invert(w: String) -> String {
    let mut str_vec: Vec<char> = w.chars().collect();
    str_vec.rotate_left(1);
    str_vec.iter().collect::<String>()
}

fn comparewords(w1: &str, w2: &str) -> (bool, u8) {
    let wordlen = w1.len();
    let mut invword: String = w2.to_string().clone();
    if (wordlen != w2.len()) || (wordlen < 4) || (w1 == w2){ 
        // println!("A");
        return (false, 0)
    }
    else {
        // println!("B");
        for i in 0..wordlen {
            if invword == w1{
                return (true, (i as u8))
            }
            invword = invert(invword);
        }
    }

    return (false, 0)
}

fn main() {
    let file = File::open("words_alpha.txt").unwrap();
    let mut words = getlist(file).unwrap();
    let mut compwords = words.clone();
    let mut comp: (bool, u8);

    words.drain(..306995);

    for i in &words {
        for j in &compwords {
            comp = comparewords(i, j);
            if comp.0 {
                print!("{} ", i);
                println!("{}", j);
            }
        }
        // compwords.remove(0);
    }
}
