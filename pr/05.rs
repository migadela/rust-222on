fn main() {
    const W: u32 = 30; 
    const H: u32 = 15; 

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;

            
            let mid = H / 2;
            
            let inner_y = if y <= mid { y } else { H - 1 - y };

            
            let left_diag = inner_y * 2;
            let right_diag = (W - 1) - inner_y * 2;

            let is_left_diag = x == left_diag;
            let is_right_diag = x == right_diag;

            let c = if is_horizontal || is_vertical || is_left_diag || is_right_diag {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}
