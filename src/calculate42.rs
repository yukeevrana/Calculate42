use regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Oper {
    Add,
    Sub,
    Multiply,
    Divide,
    Operand(u32)
}

impl Oper {
    fn get_priority(&self) -> u8 {
        match self {
            Oper::Add => 1,
            Oper::Sub => 1,
            Oper::Multiply => 2,
            Oper::Divide => 2,
            _ => 0
        }
    }
}

pub struct Calc {}

impl Calc {
    pub fn new() -> Self {
        Calc {}
    }

    /// Checks if string is a valid math expression 
    fn is_math_expr(message: &String) -> bool {
        let re = regex::Regex::new(r"^[\d\s\+\-\*/%\(\)]+$").unwrap(); // Numbers, whitespaces, +, -, *, /, %, (, )

        re.is_match(message.as_str())
    }

    /// Converts a string with a *valid* (but not necessarily correct) math expression 
    /// to a stack with an expression in RPN. Tests will show in detail.
    fn convert(&self, math_expr: &String) -> Vec<Oper> {
        let mut result: Vec<Oper> = Vec::new();
        let mut temp: Vec<Oper> = Vec::new();
        let mut operand = String::new();

        for current_ch in math_expr.replace(" ", "").chars() {
            match current_ch {
                operation_symbol if operation_symbol == '+' || operation_symbol == '-' || operation_symbol == '*' || operation_symbol == '/' => {
                    // If found an operation symbol, the previous number has ended, so we will add it to result
                    if operand != "" {
                        result.push(Oper::Operand(operand.parse().unwrap())); // We check it below when filling
                        operand.clear();
                    } 

                    let current_operation = match operation_symbol {
                        '+' => Oper::Add,
                        '-' => Oper::Sub,
                        '*' => Oper::Multiply,
                        _ => Oper::Divide // We don't need to check, the main check in the 'if' above
                    };

                    loop {
                        match temp.last() {
                            Some(last_operation) if last_operation.get_priority() >= current_operation.get_priority() => {
                                result.push(temp.pop().unwrap()); // The last element is definitely exists
                            }
                            _ => { break; }
                        }
                    }
                    temp.push(current_operation);
                }
                number => {
                    operand.push(number); // If a char is not an operation symbol, it is a number, this fn doesn't check
                }
            }
        }
        
        // Don't forget the last number
        if operand != "" {
            result.push(Oper::Operand(operand.parse().unwrap())); // We don't check
        }

        // Don't forget operations on the stack
        temp.reverse();
        for oper in temp {
            result.push(oper);
        }

        result
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
