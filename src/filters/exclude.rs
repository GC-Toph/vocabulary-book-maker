mod hamming;

use hamming::Hamming;
use rayon::prelude::*;
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
    let ret = if threshold == 0.0 {
        words
            .into_par_iter()
            .filter(|word| word_test(word.as_bytes(), &words3500))
            .collect()
    } else {
        words
            .into_par_iter()
            .filter(|word| {
                word_hamming_test(word.as_bytes(), &words3500, threshold, &mut Hamming::new())
            })
            .collect()
    };
    Ok(ret)
}

fn word_hamming_test(word: &[u8], words3500: &String, threshold: f64, ham: &mut Hamming) -> bool {
    for learned in words3500.lines() {
        let learned = learned.as_bytes();
        if (ham.dist(learned, word) as f64) / (max(learned.len(), word.len()) as f64) <= threshold {
            return false;
        }
    }
    return true;
}

// threshold == 0 as f64
fn word_test(word: &[u8], words3500: &String) -> bool {
    for learned in words3500.lines() {
        if learned.as_bytes() == word {
            return false;
        }
    }
    return true;
}
