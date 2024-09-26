// Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}

1way
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x 
    };

    assert_eq!(v, 3);

    println!("Success!");
}

2way
fn main() {
    let v = calculate_value(); 
    assert_eq!(v, 3);

    println!("Success!");
}

fn calculate_value() -> i32 {
    let mut x = 1;
    x += 2; 
    x 
}
