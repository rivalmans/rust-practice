fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];

    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}

fn main() {
    let num = 1221;
    if is_palindrome(num) {
        println!("Число {} є паліндромом", num);
    } else {
        println!("Число {} не є паліндромом", num);
    }
}
