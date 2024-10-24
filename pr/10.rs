pub fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }
    let n = ((n % len) + len) % len;
    let split_index = len as usize - n as usize;
    let (left, right) = s.split_at(split_index);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let shifts = [
        ("abcdefgh", 0, "abcdefgh"),
        ("abcdefgh", 8, "abcdefgh"),
        ("abcdefgh", -8, "abcdefgh"),
        ("abcdefgh", 1, "habcdefg"),
        ("abcdefgh", 2, "ghabcdef"),
        ("abcdefgh", 10, "ghabcdef"),
        ("abcdefgh", -1, "bcdefgha"),
        ("abcdefgh", -2, "cdefghab"),
        ("abcdefgh", -10, "cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(s, n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), *exp);
        });
}
