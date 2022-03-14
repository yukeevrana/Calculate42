use std::io;

mod calculate42;

fn main() {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer).expect("Input error"); 

        let string_expr = String::from(buffer.trim_end());
        if calculate42::is_math_expr(&string_expr) {
            let rpn = calculate42::convert(&string_expr);
            let result = calculate42::recursive_calculate(&rpn);
            match result {
                Some(n) => println!("{}", n),
                None =>  println!("Failed to calculate")
            }
        }
    }
}
