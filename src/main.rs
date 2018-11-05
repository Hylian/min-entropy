use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    println!("{:?}", args);

    let filename = &args[1];
    println!("Parsing file: {}", filename);

    let mut f = File::open(filename).expect("File not found!");

    let mut data: Vec<_> = Vec::new();
    f.read_to_end(&mut data).expect("Failed to read file!");

    println!("Parsed {} bytes", data.len());
    data.sort_unstable();

    let (freq_count, _, _) = data.iter().fold((0u32, 0u32, 0u8),
                                   |counter: (u32, u32, u8), x| {
                                           let (max_count, cur_count, value) = counter;
                                           if *x == value {
                                               if cur_count >= max_count {
                                                   (cur_count+1, cur_count+1, *x)
                                               } else {
                                                   (max_count, cur_count+1, *x)
                                               }
                                           } else if cur_count > max_count {
                                               (cur_count, 1, *x)
                                           } else {
                                               (max_count, 1, *x)
                                           }
                                   });
    println!("Highest repeat value count: {}", freq_count);
    println!("Total binary len: {}", data.len());
    println!("Min entropy = repeats/len = {}", ((freq_count as f64) / (data.len() as f64)));
}
