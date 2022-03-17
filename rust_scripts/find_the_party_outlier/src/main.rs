fn main() {
    let result = find_outlier(&[17,6,2]);

    println!("{}", result)
}

fn find_outlier(values: &[i32]) -> i32 {
    let iter = values.iter();

    let result: Vec<&i32>;

    if values[0] % 2 == 0 && values[1] % 2 == 0 || values[1] % 2 == 0 && values[2] % 2 == 0 || values[0] % 2 == 0 && values[2] % 2 == 0 {
        result = iter.filter(|num| *num % 2 != 0).collect();
    } else {
        result = iter.filter(|num| *num % 2 == 0).collect();
    }

    *result[0]
}

fn find_outlier_2(values: &[i32]) -> i32 {
    let sum: i32 = values.iter()
        .take(3)
        .map(|n| n.abs() % 2)
        .sum();

    let m = if sum == 1 || sum == 0 { 1 } else { 0 };

    values
        .iter()
        .find(|n| n.abs() % 2 == m)
        .map(|n| *n)
        .unwrap_or(0)
}