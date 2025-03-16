const WIDTH: usize = 30;
const HEIGHT: usize = 15;

fn main() {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == y * WIDTH / HEIGHT || x + y * WIDTH / HEIGHT == WIDTH - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
