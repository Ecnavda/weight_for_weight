fn main() {
    println!("Enter weights to be sorted.");

    println!("{}", order_weight("2000 10003 1234000 44444444 9999 11 11 22 123"));
}

fn order_weight(s: &str) -> String {
    let original: Vec<&str> = s.clone().split_whitespace().collect();
    let mut modified = Vec::<(String, i32)>::new();
    let mut weight: i32;
    for y in original {
        weight = y.chars()
            .map( |a| a.to_digit(10).unwrap() as i32)
            .sum();
        modified.push((y.to_string(), weight));
    }

    modified.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        } else {
            return a.1.cmp(&b.1);
        };
    });

    let output: Vec<String> = modified.into_iter().map( |a| a.0).collect();
    output.join(" ")
}
