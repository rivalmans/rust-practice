fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;

    let mut output = String::new();

    for i in 0..HEIGHT {
        output.push_str(&" ".repeat(HEIGHT - i - 1));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    for i in (0..HEIGHT - 1).rev() {
        output.push_str(&" ".repeat(HEIGHT - i - 1));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    print!("{}", output);
}
