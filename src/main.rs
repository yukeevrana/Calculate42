use std::io;

mod calculate42;

fn main() {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer).expect("Input error"); 

        let string_expr = String::from(buffer.trim_end());

        match calculate42::try_calculate(&string_expr) {
            Some(n) => println!("{}", n),
            None => println!("Failed to calculate")
        }
    }
}
