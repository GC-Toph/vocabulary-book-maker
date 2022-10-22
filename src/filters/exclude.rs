mod hamming;

use hamming::Hamming;
use std::cmp::max;
use std::error::Error;
use std::{fs, io};

pub fn exclude(words: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let words_exclude = fs::read_to_string("exclude.txt")?;
    let words_exclude: Vec<&str> = words_exclude.split("\n").collect();

    let mut ret: Vec<String> = Vec::new();
    for word in words {
        for ex_word in words_exclude.iter().copied() {
            if word.as_str() == ex_word {
                break;
            }
        }
        ret.push(word);
    }
    Ok(ret)
}

pub fn exclude_hamming(words: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let words3500 = fs::read_to_string("exclude_3500.txt")?;
    let words3500: Vec<&str> = words3500.split("\n").collect();

    println!("Please Input threshold of Hamming filter in 3500words (defalut as 0.24): ");
    let mut threshold = 0.24;
    let mut threshold_str = String::new();
    io::stdin().read_line(&mut threshold_str)?;
    if threshold_str.trim() != "" {
        threshold = threshold_str.trim().parse()?;
    }

    let mut ham = Hamming::new();

    let mut ret: Vec<String> = Vec::new();
    for word in words {
        if word_hamming_test(&word, &words3500, threshold, &mut ham) {
            ret.push(word);
        }
    }
    Ok(ret)
}

fn word_hamming_test(word: &str, words3500: &Vec<&str>, threshold: f64, ham: &mut Hamming) -> bool {
    for learned in words3500.iter().copied() {
        if threshold == 0 as f64 {
            return learned != word;
        } else if (ham.dist(learned, word) as f64) / (max(learned.len(), word.len()) as f64)
            <= threshold
        {
            return false;
        }
    }
    return true;
}
