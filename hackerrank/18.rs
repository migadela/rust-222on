use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Calculate the total cost excluding the item at index k
    let actual_share: i32 = bill.iter().enumerate()
        .filter(|&(index, _)| index as i32 != k)
        .map(|(_, &cost)| cost)
        .sum::<i32>() / 2;

    // Check if the charged amount matches the actual share
    if b == actual_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - actual_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}
