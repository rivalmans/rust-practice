use std::env;

fn draw_tree(num_triangles: usize) {
    let mut max_width = num_triangles * 2 + 1; 

    for i in 0..num_triangles {
        let height = i + 2;
        (0..height).for_each(|j| {
            let stars = 1 + j * 2;
            let spaces = (max_width - stars) / 2;
            println!("{:spaces$}{}", "", "*".repeat(stars), spaces = spaces);
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let num_triangles = args.get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(5);

    draw_tree(num_triangles);
}
