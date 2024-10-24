use rand::Rng;

// Функція для генерації рандомного вектора
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для знаходження мінімальної пари у векторі
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32, usize)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);
    let mut min_index = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
            min_index = i;
        }
    }

    Some((min_pair.0, min_pair.1, min_index))
}

// Функція для виводу вектора у зрозумілому вигляді
fn print_vector(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3} ", i);
    }
    println!("");
    println!("data:    {:?}", data);
}

fn main() {
    // Генеруємо вектор
    let vector = gen_random_vector(20);
    
    // Виводимо вектор
    print_vector(&vector);
    
    // Знаходимо та виводимо мінімальну пару
    if let Some((a, b, index)) = min_adjacent_sum(&vector) {
        println!("indexes:");
        print!("         ");
        for _ in 0..index {
        println!("   \\");
        print!("         ");
        for _ in 0..index {
        println!("__ __/");
        println!("min adjacent sum={}+{}={} at indexes:{},{}", a, b, a + b, index, index + 1);
    } else {
        println!("The vector is too short to find adjacent pairs.");
    }
}
