// Функція, що обчислює мінімальну кількість кроків для вирівнювання вантажу на кораблях
fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    
    // Якщо загальний вантаж не може бути рівномірно поділений між усіма кораблями, це неможливо
    if total as usize % n != 0 {
        return -1; // -1 означає, що вирівнювання неможливе
    }
    
    let target_load = total / n as u32;
    let mut moves = 0;
    let mut current_balance = 0;

    for &cargo in shipments.iter() {
        current_balance += cargo as i32 - target_load as i32;
        if current_balance != 0 {
            moves += 1;
        }
    }
    
    moves as isize
}

// Функція, що генерує Vec<u32>, який можна рівномірно розподілити між усіма кораблями
fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let base: u32 = rng.gen_range(10..100);
    let mut shipments: Vec<u32> = Vec::new();
    for _ in 0..n {
        shipments.push(base);
    }
    shipments
}

// Приклад використання
fn main() {
    let shipments = vec![5, 1, 1, 1, 6];
    // Пояснення прикладу:
    // [5, 1, 1, 1, 6]
    // загальна вага = 5 + 1 + 1 + 1 + 6 = 14
    // середнє = 14 / 5 = 2.8 (не є цілим числом, тому вирівнювання неможливе)
    let moves = count_permutation(&shipments);
    println!("Мінімальна кількість кроків для вирівнювання: {}", moves);
    
    let generated_shipments = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", generated_shipments);

    let example_shipments = vec![8, 2, 2, 4, 4];
    // Пояснення прикладу:
    // [8, 2, 2, 4, 4]
    // загальна вага = 8 + 2 + 2 + 4 + 4 = 20
    // середнє = 20 / 5 = 4
    // [8, 2, 2, 4, 4]
    // -2 +2 (переміщення 2 від 8 до 2)
    // -2 +2 (переміщення 2 від 8 до 2)
    // відповідь = 4 кроки
    let example_moves = count_permutation(&example_shipments);
    println!("Мінімальна кількість кроків для вирівнювання для прикладу: {}", example_moves);

    let example_shipments_2 = vec![9, 3, 7, 2, 9];
    // Пояснення прикладу:
    // [9, 3, 7, 2, 9]
    // загальна вага = 9 + 3 + 7 + 2 + 9 = 30
    // середнє = 30 / 5 = 6
    // [9, 3, 7, 2, 9]
    // -3 +3 (переміщення 3 від 9 до 3)
    // -1 +1 (переміщення 1 від 7 до 2)
    // +3 -3 (переміщення 3 від 9 до 7)
    // відповідь = 7 кроків
    let example_moves_2 = count_permutation(&example_shipments_2);
    println!("Мінімальна кількість кроків для вирівнювання для другого прикладу: {}", example_moves_2);
}
