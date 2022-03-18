fn main() {
    let result = maskify("1");

    println!("{}", result);
}

fn maskify(cc: &str) -> String {
    if cc.len() < 5 {
        return cc.to_string();
    }

    let mut result = std::iter::repeat("#")
        .take(cc.len() - 4)
        .collect::<String>();

    result.push_str(&cc[cc.len() - 4..]);

    result
}
