extern crate itertools;

use itertools::join;

const MAX_BOTTLES_OF_BEER: i32 = 99;

pub fn verse(n: i32) -> String {

    let bottles = bottles_txt(n);
    let take_bottle = take_bottle_txt(n);
    let bottles_minus_one = bottles_txt(n - 1);

    format!("{bottles_title} of beer on the wall, {bottles} of beer.\n\
    {take_bottle}, {bottlesminusone} of beer on the wall.\n",
            bottles_title=&uppercase_first_char(&bottles),
            bottles=bottles,
            take_bottle=take_bottle,
            bottlesminusone=bottles_minus_one).to_string()
}

pub fn sing(start: i32, end: i32) -> String {
    let verses_iter = (end..start + 1)
        .rev()
        .map(|i| verse(i));

    join(verses_iter, "\n")
}

fn take_bottle_txt(n: i32) -> String {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around"
    }.to_string()
}

fn bottles_txt(n: i32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ if n < 0 => format!("{} bottles", MAX_BOTTLES_OF_BEER).to_string(),
        _ => format!("{} bottles", n).to_string()
    }
}

fn uppercase_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
        None => String::new(),
    }
}
