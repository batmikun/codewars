fn main() {
    let result = solution("camelCaseCasingH");

    println!("{}", result);
}

fn solution(s: &str) -> String {
    let mut start = 0;
    let mut find = 0;
    let mut result: String = String::new();

    for char in s.chars() {
        
        if char.is_uppercase() {
            result.push_str(&s[start..find].to_string());
            result.push_str(" ");
            start = find;
        }

        find = find + 1;
    }

    result.push_str(&s[start..]);

    result
    
}