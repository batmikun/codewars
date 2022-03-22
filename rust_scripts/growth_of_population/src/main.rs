fn main() {
    let nb_year = nb_year(1500, 5.0, 100, 5000);

    println!("{}", nb_year);
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut years = 0;

    let mut population = p0;

    while population < p {
        years += 1;

        let percentage = (population as f64 * percent) / 100.0;
        let new_population = percentage as i32 + aug;

        population += new_population;
        println!("{}", population);
    }

    years
}
