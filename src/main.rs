extern crate natural;
extern crate rayon;
use natural::tokenize::tokenize;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read};

fn process_word(x: &str) -> String {
    x.trim()
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_alphanumeric() || ch == '\'')
        .collect::<String>()
}

fn canonicalize(content: &str, stopwords: &Vec<String>) -> Vec<String> {
    tokenize(&content)
        .iter()
        .map(|x| process_word(x))
        .filter(|x| stopwords.iter().all(|y| y != x) && x.len() > 1)
        .collect::<Vec<_>>()
}

fn read_file(filename: &str) -> String {
    let mut content = String::new();
    File::open(filename)
        .expect("File not found")
        .read_to_string(&mut content)
        .expect("Something went wrong reading the file");
    content
}

fn main() {
    let stdin = io::stdin();
    let filenames: Vec<_> = stdin.lock().lines().filter_map(|x| x.ok()).collect();

    let stopwords = read_file("stopwords.txt");
    let stopwords = stopwords.lines().map(|x| process_word(x)).collect::<Vec<_>>();

    eprintln!("Reading files...");
    let contents: Vec<_> = filenames
        .iter()
        .map(|filename| read_file(filename))
        .collect();

    eprintln!("Canonicalizing...");
    let mut result = contents
        .par_iter()
        .flat_map(|content| canonicalize(&content, &stopwords))
        .collect::<Vec<_>>();

    eprintln!("Sorting {} words...", result.len());
    result.par_sort_unstable();

    eprintln!("Counting words...");
    let mut index = 0;
    let mut word_count = HashMap::new();
    while index < result.len() {
        let mut count = 1;
        while index + 1 < result.len() && result[index] == result[index+1] {
            index += 1;
            count += 1;
        }
        word_count.insert(&result[index], count);
        index += 1;
    }

    let mut out = word_count.par_iter().collect::<Vec<_>>();
    out.par_sort_unstable_by(|a, b| {
        match a.1.cmp(b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(b.0),
            x => x
        }
    });

    for (word, count) in out {
        println!("{} {}", count, word);
    }
}
