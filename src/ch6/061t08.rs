//1
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
}
//2
fn main() {
    let s = "hello, world";
    let s1: &str = s;
}
//3
fn main() {
    let s = "hello, world".to_string();
    let s1: String = s;
}
