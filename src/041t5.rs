
// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = u8::checked_add(251, 8).unwrap(u8::MAX);
   println!("{},{}",v1,v2);
}
