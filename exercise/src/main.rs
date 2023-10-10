use std::{
    collections::HashMap,
    error::Error,
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let result = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/dataset"))
        .unwrap()
        .map(|d| d.unwrap().path())
        .flat_map(|path| {
            let file = File::open(path);
            let reader = BufReader::new(file.unwrap());
            reader.lines()
        })
        .map(|l| {
            if let Ok(line) = l {
                let words = line.split(' ');
                // thread::sleep(Duration::from_millis(100));
                let mut counts = HashMap::new();
                words.for_each(|w| *counts.entry(w.to_string()).or_insert(0) += 1);
                counts
            }
        })
        .fold(HashMap::new(), |mut acc, words| {
            words
                .iter()
                .for_each(|(k, v)| *acc.entry(k.clone()).or_insert(0) += v);
            acc
        });

    println!("{:?}", result);

    println!("{:?}", start.elapsed());
    Ok(())
}
