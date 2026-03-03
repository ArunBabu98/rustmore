mod bigram;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::bigram::Bigram;

fn main() -> std::io::Result<()> {
    let file = File::open("./assets/names.txt")?;
    let reader = BufReader::new(file);
    let names: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    println!("{:?}",&names[..10]);
    println!("Total number of names: {}",names.len());
    let bigram = Bigram::new(names);
    let itr: Vec<_> = bigram.pairs().collect();
     println!("{:?}",&itr[..10]);
    Ok(())
}
