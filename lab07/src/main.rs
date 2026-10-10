
use rand::RngExt;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    println!("Highs");
    for (day, temp) in DAYS.iter().zip(highs.iter()) {
        println!("{day}: {temp}");
    }

    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    println!("Updated Highs");
    for (day, temp) in DAYS.iter().zip(highs.iter()) {
        println!("{day}: {temp}");
    }

    let average = average_temp(&highs);
    let hottest = hottest_day(&highs);
    let above = count_above(&highs, 75);

    println!("Average temperature: {:.2}", average);
    println!("Hottest day: {}", DAYS[hottest]);
    println!("Days over 75: {}", above);
}

fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }

    let sum: i32 = log.iter().sum();
    sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<i32>) -> usize {
    let mut hottest = 0;

    for i in 1..log.len() {
        if log[i] > log[hottest] {
            hottest = i;
        }
    }

    hottest
}

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
    log.iter().filter(|&&temp| temp > threshold).count()
}
