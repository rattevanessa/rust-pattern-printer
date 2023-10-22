trait Pattern {
    fn draw(&self, c: char);
}

struct BoxPattern {
    width: i32,
    height: i32,
}

impl BoxPattern {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl Pattern for BoxPattern {
    fn draw(&self, c: char) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!("{}", c)
            }
            println!()
        }
    }
}

fn main() {
    let bp = BoxPattern::new(32, 4);
    bp.draw('#');
}
