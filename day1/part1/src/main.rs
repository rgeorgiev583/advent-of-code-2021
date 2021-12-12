use std::{
    error::Error,
    io::{stdin, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = BufReader::new(stdin);
    let mut previous_measurement: Option<usize> = None;
    let mut num_increments: usize = 0;
    for line in stdin.lines() {
        let line = line?;
        let current_measurement = line.parse::<usize>()?;
        if let Some(previous_measurement) = previous_measurement {
            if current_measurement > previous_measurement {
                num_increments += 1;
            }
        }
        previous_measurement = Some(current_measurement);
    }
    println!("{}", num_increments);
    Ok(())
}
