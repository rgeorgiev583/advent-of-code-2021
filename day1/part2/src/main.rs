use std::{
    error::Error,
    io::{stdin, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();
    let mut sliding_window: [usize; 3] = [0; 3];
    if let Some(line) = lines.next() {
        let current_measurement = line?.parse::<usize>()?;
        sliding_window[0] = current_measurement;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "first measurement was not provided in input",
        )));
    }
    if let Some(line) = lines.next() {
        let current_measurement = line?.parse::<usize>()?;
        sliding_window[1] = current_measurement;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "second measurement was not provided in input",
        )));
    }
    let mut previous_sum: Option<usize> = None;
    let mut num_increments: usize = 0;
    for line in lines {
        let line = line?;
        let current_measurement = line.parse::<usize>()?;
        sliding_window[2] = current_measurement;
        let current_sum: usize = sliding_window.into_iter().sum();
        if let Some(previous_sum) = previous_sum {
            if current_sum > previous_sum {
                num_increments += 1;
            }
        }
        previous_sum = Some(current_sum);
        sliding_window.rotate_left(1);
    }
    println!("{}", num_increments);
    Ok(())
}
