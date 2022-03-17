use std::ops::Add;

fn main() {
    println!("Hello, world!");

    croquet_club(vec![(45, 12), (55,21), (19, -2), (104, 20)]);
}

fn croquet_club(data: Vec<(i32, i32)>) -> Vec<String> {
    let data_iterator = data.iter();

    let mut status: Vec<String> = vec![];

    for members in data_iterator {
        if members.0 >= 55 && members.1 > 7 {
            status.push(String::from("Senior"));
        } else {
            status.push(String::from("Open"));
        }
    }

    println!("{:?}", status);

    status
}