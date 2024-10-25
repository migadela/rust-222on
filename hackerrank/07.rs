use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i64]) {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    let total_sum: i64 = sorted_arr.iter().sum();
    let min_sum = total_sum - sorted_arr[sorted_arr.len() - 1];
    let max_sum = total_sum - sorted_arr[0];

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
