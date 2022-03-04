use std::io;

mod calculate42;

fn main() {
    let mut calc = calculate42::Calc::new();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer).expect("Input error"); 

        match calc.take_message(String::from(buffer.trim_end())) {
            Ok(n) => { println!("{:?}", n) },
            Err(error) => { 
                match error {
                    "exit" => break,
                    _ => println!("{error}")
                } 
            }
        }
    }
}
