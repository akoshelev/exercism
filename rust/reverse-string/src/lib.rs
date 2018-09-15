extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;


pub fn reverse(input: &str) -> String {
    grapheme_reverse(input)
}

#[allow(dead_code)]
fn naive_reverse(input: &str) -> String {
    let mut res = String::new();

    // ?? why mut??
    let mut iter = input.chars();
    while let Some(ch) = iter.next_back() {
        res.push(ch)
    }

    res
}

#[allow(dead_code)]
fn immut_reverse(input: &str) -> String {
    input.chars()
        .into_iter()
        .rev()
        .collect()
}

fn grapheme_reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .into_iter()
        .rev()
        .collect()
}

