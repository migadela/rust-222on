fn draw_christmas_tree(n_triangles: usize) {
    let max_width = n_triangles * 2 + 1;

    
    for triangle in 1..=n_triangles {
        
        for stars_count in (1..=2 * triangle - 1).step_by(2) {
            
            let spaces_count = (max_width - stars_count) / 2;
            let spaces = " ".repeat(spaces_count);
            let stars = "*".repeat(stars_count);

            
            println!("{}{}{}", spaces, stars, spaces);
        }
    }
}

fn main() {
    let n_triangles: usize = 6; 
    draw_christmas_tree(n_triangles);
}

