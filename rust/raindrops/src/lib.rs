pub fn raindrops(n: usize) -> String {
    let tuples = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    // find all matches if they exist
    let matches: String = tuples
        .iter()
        .filter(|(num, _)| n % num == 0)
        .map(|&(_, s)| s)
        .collect();

    Some(matches)
        .filter(|s| !s.is_empty())
        .unwrap_or(n.to_string())
}

#[allow(dead_code)]
fn raindrops_mut(n: usize) -> String {

    let mut res = String::new();

    if n % 3 == 0 { res.push_str("Pling") }
    if n % 5 == 0 { res.push_str("Plang") }
    if n % 7 == 0 { res.push_str("Plong") }

    if res.len() == 0 { res = n.to_string() }

    res
}
