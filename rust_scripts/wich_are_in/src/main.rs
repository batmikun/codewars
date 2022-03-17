fn main() {
    let arr_a = &["xyz", "live", "strong"];

    let arr_b = &["lively", "alive", "harp", "sharp", "armstrong"];

    let result = in_array(arr_a, arr_b);

    println!("{:?}", result)
}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut return_vec: Vec<String> = vec![];

    for substring in arr_a.iter() {
        for word in arr_b.into_iter() {
            if word.contains(substring) {
                if !return_vec.contains(&substring.to_string()){
                    return_vec.push(substring.to_string());
                }
            }
        }
    }

    return_vec.sort();

    return_vec
}