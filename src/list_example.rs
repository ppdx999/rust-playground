fn calc_mean(list: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in list.iter() {
        sum += i;
    }
    sum as f32 / list.len() as f32
}

fn calc_median(list: &Vec<i32>) -> f32 {
    let mut list = list.clone();
    list.sort();
    let len = list.len();
    if len % 2 == 0 {
        (list[len / 2] + list[len / 2 - 1]) as f32 / 2.0
    } else {
        list[len / 2] as f32
    }
}

fn calc_mode(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort();
    let mut mode = 0;
    let mut mode_count = 0;
    let mut count = 0;
    let mut prev = 0;
    for i in list.iter() {
        if *i == prev {
            count += 1;
        } else {
            count = 1;
        }
        if count > mode_count {
            mode = *i;
            mode_count = count;
        }
        prev = *i;
    }
    mode
}

pub fn main() {
    let list = [1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    let list = Vec::from(list);

    let mean = calc_mean(&list);
    let median = calc_median(&list);
    let mode = calc_mode(&list);

    // mean
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("mode: {}", mode);
}
