mod hamming;

use hamming::Hamming;
use std::cmp::max;
use std::error::Error;
use std::{fs, io};

pub fn exclude(words: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let words_exclude = fs::read_to_string("exclude.txt")?;

    let ret: Vec<_> = words
        .into_iter()
        .filter(|word| {
            for ex_word in words_exclude.lines() {
                if word.as_str() == ex_word {
                    return false;
                }
            }
            return true;
        })
        .collect();
    Ok(ret)
}

pub fn exclude_hamming(words: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let words3500 = fs::read_to_string("exclude_3500.txt")?;

    println!("Please Input threshold of Hamming filter in 3500words (defalut as 0.24): ");
    let mut threshold = 0.24;
    let mut threshold_str = String::new();
    io::stdin().read_line(&mut threshold_str)?;
    if threshold_str.trim() != "" {
        threshold = threshold_str.trim().parse()?;
    }

    let mut ham = Hamming::new();

    let ret: Vec<_> = words
        .into_iter()
        .filter(|word| word_hamming_test(&word.chars().collect(), &words3500, threshold, &mut ham))
        .collect();
    Ok(ret)
}

fn word_hamming_test(
    word: &Vec<char>,
    words3500: &String,
    threshold: f64,
    ham: &mut Hamming,
) -> bool {
    for learned in words3500.lines() {
        let learned: &Vec<char> = &learned.chars().collect();

        if threshold == 0 as f64 {
            if learned == word {
                return false;
            }
        } else {
            if (ham.dist(learned, word) as f64) / (max(learned.len(), word.len()) as f64)
                <= threshold
            {
                return false;
            }
        }
    }
    return true;
}
