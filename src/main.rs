use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, ErrorKind};
use itertools::Itertools;

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let buffer = BufReader::new(input);

    let v: Vec<i64> = buffer.lines().map(|l| l.and_then(|v| v.parse::<i64>()?)).collect()?;

    for vpair in v.iter().combinations(2) {
        println!("{:?}", vpair);
    }
    println!("{:?}", v);
    Ok(())
}
