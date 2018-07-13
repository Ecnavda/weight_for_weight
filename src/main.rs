fn main() {
    println!("Enter weights to be sorted.");

    println!("{}", order_weight("103 123 4444 99 2000"));
}

fn order_weight(s: &str) -> String {
    let original: Vec<&str> = s.clone().split_whitespace().collect();
    let mut modified = Vec::<(String, i32)>::new();
    let mut weight: i32;
    for y in original {
        weight = 0;
        weight = y.chars()
            .map( |a| a.to_digit(10).unwrap() as i32)
            .sum();
        modified.push((y.to_string(), weight));
    }

    modified.sort_by_key(|a| a.1);

    let output: Vec<String> = modified.into_iter().map( |a| a.0).collect();
    output.join(" ")
}
