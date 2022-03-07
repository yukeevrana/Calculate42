// use colored::Colorize;
use regex;
use std::collections::HashMap;

pub struct Calc {
    // // output_color: String,
    // input: String,
    // // commands: Vec<String>,
    // output: String,
    priorities: HashMap<Oper, u8>
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Oper {
    Operand(u32),
    Add,
    Sub,
    Multiply,
    Divide
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            // // output_color: String::from("white"),
            // input: String::new(),
            // // commands: Vec::new(),
            // output: String::new(),
            priorities: HashMap::from([
                (Oper::Add, 1),
                (Oper::Sub, 1),
                (Oper::Multiply, 2),
                (Oper::Divide, 2)
            ])
        }
    }

    // pub fn take_message(&mut self, message: String) -> Result<String, &str> {
    //     self.input = message.clone();

    //     if self.input.as_str() == "exit" { return Err("exit"); }
        
    //     // self.find_color_command();

    //     match self.find_math_expr() {
    //         Err(_) => {},
    //         Ok(_) => {}
    //     };
    //     Err("")
    //     // Ok(self.get_answer())
    // }

    fn is_math_expr(message: &String) -> bool {
        let re = regex::Regex::new(r"^[\d\s\+\-\*/%\(\)]+$").unwrap();

        re.is_match(message.as_str())
    }

    fn convert(&self, math_expr: &String) -> Vec<Oper> {
        // let mut res = String::new();
        let mut res: Vec<Oper> = Vec::new();
        let mut tmp: Vec<Oper> = Vec::new();
        let mut operand = String::new();

        for ch in math_expr.replace(" ", "").chars() {
            match ch {
                operation if operation == '+' || operation == '-' || operation == '*' || operation == '/' => {
                    if operand != "" {
                        res.push(Oper::Operand(operand.parse().unwrap()));
                        operand.clear();
                    } 

                    let oper = match operation {
                        '+' => Oper::Add,
                        '-' => Oper::Sub,
                        '*' => Oper::Multiply,
                        _ => Oper::Divide
                    };

                    loop {
                        match tmp.last() {
                            Some(value) if self.priorities.get(value).unwrap() >= self.priorities.get(&oper).unwrap() => {
                                res.push(tmp.pop().unwrap());
                            }
                            _ => { break; }
                        }
                    }
                    tmp.push(oper);
                }
                number => {
                    let n = ch.to_digit(10);
                    match n {
                        None => {},
                        _ => { 
                            operand.push(number); 
                        }
                    }
                }
            }
        }
        
        if operand != "" {
            res.push(Oper::Operand(operand.parse().unwrap()));
        }
        tmp.reverse();
        for oper in tmp {
            res.push(oper);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_math_expr_looks_like_math_expr_without_whitespaces() {
        use super::*;

        for message in ["2+2", "3*3", "4/4", "5-5", "1**1", "6//6", "7%7", ")8(8"] {
            assert!(Calc::is_math_expr(&String::from(message)));
        }
    }

    #[test]
    fn is_math_expr_looks_like_math_expr_with_whitespaces() {
        use super::*;

        for message in ["2 + 2", "3 * 3", "4 /4", "5- 5", "1* *1", "6    //6", "7%   7", ") 8(    8"] {
            assert!(Calc::is_math_expr(&String::from(message)));
        }
    }

    #[test]
    fn is_math_expr_definitely_not_math_expr() {
        use super::*;

        for message in ["2 + 2f", "3 kk* 3", "4 !/4", "5- ?5", "1* nana*1", "word", "another word", ""] {
            assert_eq!(Calc::is_math_expr(&String::from(message)), false);
        }
    }

    #[test]
    fn convert_numbers() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        assert_eq!(calc.convert(&String::from("2387")), res)
    }

    #[test]
    fn convert_numbers_with_whitespaces() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        assert_eq!(calc.convert(&String::from("2 3 87")), res)
    }

    #[test]
    fn convert_numbers_with_plus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_two_operations_in_a_row() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Add);
        res.push(Oper::Operand(495));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 ++ 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_extra_operation() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Add);
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5+")), res)
    }

    #[test]
    fn convert_numbers_with_two_pluses_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Add);
        res.push(Oper::Operand(43021));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_minus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Sub);
        assert_eq!(calc.convert(&String::from("2 3 87 - 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_minus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Add);
        res.push(Oper::Operand(43021));
        res.push(Oper::Sub);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5- 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_minus_and_plus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Sub);
        res.push(Oper::Operand(43021));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 - 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_multiply_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Multiply);
        assert_eq!(calc.convert(&String::from("2 3 87 * 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_multiply_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Operand(43021));
        res.push(Oper::Multiply);
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_multiply_and_plus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Multiply);
        res.push(Oper::Operand(43021));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 * 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_divide_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Divide);
        assert_eq!(calc.convert(&String::from("2 3 87 / 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_divide_and_multiply_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Divide);
        res.push(Oper::Operand(43021));
        res.push(Oper::Multiply);
        assert_eq!(calc.convert(&String::from("2 3 87 / 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_divide_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Operand(43021));
        res.push(Oper::Divide);
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 + 49 5/ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_divide_and_plus_correct() {
        use super::*;

        let calc = Calc::new();

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387));
        res.push(Oper::Operand(495));
        res.push(Oper::Divide);
        res.push(Oper::Operand(43021));
        res.push(Oper::Add);
        assert_eq!(calc.convert(&String::from("2 3 87 / 49 5+ 43 0 21")), res)
    }
}
