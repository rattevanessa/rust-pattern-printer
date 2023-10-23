trait Pattern {
    fn draw(&self, c: char);
}

struct ReverseTrianglePattern(i32);

struct TrianglePattern(i32);

struct BoxPattern {
    width: i32,
    height: i32,
}

struct BorderBoxPattern {
    width: i32,
    height: i32,
}

impl BoxPattern {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl BorderBoxPattern {
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

impl Pattern for BorderBoxPattern {
    fn draw(&self, c: char) {
        for i in 0..self.height {
            for j in 0..self.width {
                if i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1 {
                    print!("{}", c)
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

impl Pattern for TrianglePattern {
    fn draw(&self, c: char) {
        for i in 1..self.0 + 1 {
            for _ in 0..i {
                print!("{}", c)
            }
            println!()
        }
    }
}

impl Pattern for ReverseTrianglePattern {
    fn draw(&self, c: char) {
        for i in 1..self.0 + 1 {
            for _ in 0..i {
                print!(" ");
            }
            for _ in i..self.0 {
                print!("{}", c)
            }
            println!()
        }
    }
}

fn main() {
    let bp = BoxPattern::new(32, 4);
    bp.draw('#');
    let bbp = BorderBoxPattern::new(32, 4);
    bbp.draw('#');
    let tp = TrianglePattern(10);
    tp.draw('#');
    let rtp = ReverseTrianglePattern(10);
    rtp.draw('#');
}
