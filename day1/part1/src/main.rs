use std::{
    error::Error,
    io::{stdin, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = BufReader::new(stdin);
    let mut previous: Option<usize> = None;
    let mut num_increments: usize = 0;
    for line in stdin.lines() {
        let current = line?.parse::<usize>()?;
        if let Some(previous) = previous {
            if current > previous {
                num_increments += 1;
            }
        }
        previous = Some(current);
    }
    println!("{}", num_increments);
    Ok(())
}
