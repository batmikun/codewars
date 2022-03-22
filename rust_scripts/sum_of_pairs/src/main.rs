use std::collections::HashMap;

fn main() {
    let _result = match sum_pairs(&[1, 4, 8, 7, 3, 15], 8) {
        Some(pair) => println!("{:?}", pair),
        None => println!("There was none values matching that sum"),
    };
}

// This is not performant.
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut complement_dict: HashMap<i8, i8> = HashMap::new();

    for i in ints.into_iter() {
        if complement_dict.contains_key(i) {
            return Some((s - i, *i));
        } else {
            complement_dict.insert(s - i, *i);
        }
    }

    return None;
}
