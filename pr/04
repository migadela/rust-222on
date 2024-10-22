fn main() {
    const SIZE: usize = 5; 

    let mut output = String::new();

    for i in 0..(2 * SIZE + 1) {
        let stars = if i <= SIZE { i } else { 2 * SIZE - i };
        let spaces = SIZE - stars;
        output += &" ".repeat(spaces);
        output += &"*".repeat(2 * stars + 1);
        output += "\n";
    }

    print!("{}", output);
    println!();
}
