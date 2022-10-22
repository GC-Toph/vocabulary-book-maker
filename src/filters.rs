pub mod exclude;

use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;

const MIN_WORD_LENGTH: usize = 5;
const MAX_WORD_LENGTH: usize = 20;

pub fn main_filter(origin_text: String) -> Result<Vec<String>, Box<dyn Error>> {
    //use regex to filter dirty string
    let re = Regex::new(r"[^A-Za-z]+").unwrap();
    let ret = re.replace_all(&origin_text, " ").to_lowercase();

    let re = Regex::new(r"[a-z]+ing ").unwrap();
    let ret = re.replace_all(ret.as_str(), " ");

    // let re = Regex::new(r"[a-z]+ed ").unwrap();
    // let re = Regex::new(r"[a-z]+ly ").unwrap();
    // let re = Regex::new(r"[a-z]+s ").unwrap();
    let ret: Vec<&str> = ret.split(' ').collect();

    //use hash map to stat freq
    let mut hm: HashMap<String, u32> = HashMap::new();
    let mut tot_num: u64 = 0;
    for ele in ret {
        if ele.len() >= MIN_WORD_LENGTH && ele.len() <= MAX_WORD_LENGTH {
            let cnt = hm.entry(String::from(ele)).or_insert(0);
            *cnt += 1;
            tot_num += 1;
        }
    }

    //estimate appropritate freq
    let average_freq = tot_num as f64 / hm.len() as f64;
    println!("Average Frequency: {}", average_freq);
    let average_freq = (average_freq / 1.1).floor() as u32;
    let mut tot_num: u64 = 0;
    let mut tmp_len: u64 = 0;
    for (_, v) in &hm {
        if *v > 2 && *v <= average_freq {
            tot_num += *v as u64;
            tmp_len += 1;
        }
    }
    println!(
        "Mage Frequency for reference: {}",
        tot_num as f64 / (tmp_len + 1) as f64
    );

    //get user input of threshold freq
    println!("Please Input a Integer as the Threshold Frequency: ");
    let mut freq = String::new();
    io::stdin().read_line(&mut freq)?;
    let freq = freq.trim().parse()?;

    //construct ret value
    let mut ret: Vec<String> = Vec::new();
    for (k, v) in hm {
        if v >= freq {
            ret.push(k);
        }
    }
    return Ok(ret);
}

pub fn shuffle(words: &mut Vec<String>) -> &Vec<String> {
    let mut rng = thread_rng();
    words.shuffle(&mut rng);
    words
}
