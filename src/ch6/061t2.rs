
// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}

//1solution
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)  // Dereference Box<str> to &str
}

fn greetings(s: &str) {
    println!("{}", s)
}

//2solution
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(Box::leak(s))  // Convert Box<str> to &'static str
}

fn greetings(s: &str) {
    println!("{}", s)
}
