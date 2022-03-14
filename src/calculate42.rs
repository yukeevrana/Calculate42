use regex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Oper {
    Add,
    Sub,
    Mult,
    Div,
    Rem,
    Exp,
    // TODO: unary negation (right-associative) with priority like Exp
    Operand(f64)
}

impl Oper {
    fn get_priority(&self) -> u8 {
        match self {
            Oper::Add => 1,
            Oper::Sub => 1,
            Oper::Mult => 2,
            Oper::Div => 2,
            Oper::Rem => 2,
            Oper::Exp => 3,
            _ => 0
        }
    }
}

/// Checks if string is a valid math expression 
pub fn is_math_expr(message: &String) -> bool {
    let re = regex::Regex::new(r"^[\d\s\+\-\*/%\(\)\^]+$").unwrap(); // Numbers, whitespaces, +, -, *, /, %, (, )

    re.is_match(message.as_str())
}

/// Converts a string with a *valid* (but not necessarily correct) math expression 
/// to a stack with an expression in RPN. Tests will show in detail.
pub fn convert(math_expr: &String) -> Vec<Oper> {
    let mut result: Vec<Oper> = Vec::new();
    let mut temp: Vec<Oper> = Vec::new();
    let mut operand = String::new();

    for current_ch in math_expr.replace(" ", "").chars() {
        match current_ch {
            operation_symbol if 
                operation_symbol == '+' || operation_symbol == '-' || operation_symbol == '*' || 
                operation_symbol == '/' || operation_symbol == '%' || operation_symbol == '^' => {

                // If found an operation symbol, the previous number has ended, so we will add it to result
                if operand != "" {
                    result.push(Oper::Operand(operand.parse().unwrap())); // We check it below when filling
                    operand.clear();
                } 

                let current_operation = match operation_symbol {
                    '+' => Oper::Add,
                    '-' => Oper::Sub,
                    '*' => Oper::Mult,
                    '%' => Oper::Rem,
                    '^' => Oper::Exp,
                    _ => Oper::Div // We don't need to check, the main check in the 'if' above
                };

                loop {
                    match temp.last() {
                        Some(last_operation) if last_operation.get_priority() >= current_operation.get_priority() => {
                            result.push(*last_operation);
                            temp.pop();
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

pub fn recursive_calculate(rpn_expr: &Vec<Oper>) -> Option<f64> {
    let mut new_rpn_expr: Vec<Oper> = Vec::new();
    let mut counter: u32 = 0;
    let mut left: Option<f64> = None;
    let mut right: Option<f64> = None;
    let mut was_operation: bool = false;

    for oper in rpn_expr {

        if was_operation {
            new_rpn_expr.push(*oper);
            counter += 1;
        }
        else {
            match oper {
                Oper::Operand(n) => {
                    match left {
                        None => left = Some(*n),
                        Some(l) => {
                            if right == None {
                                right = Some(*n);
                            }
                            else {
                                new_rpn_expr.push(Oper::Operand(l));
                                counter += 1;
                                left = right;
                                right = Some(*n);
                            }
                        }
                    }
                },
                Oper::Add => {
                    match left {
                        Some(l) => {
                            match right {
                                Some(r) => {
                                    new_rpn_expr.push(Oper::Operand(l + r));
                                    left = None;
                                    right = None;
                                    counter += 1;
                                    was_operation = true;
                                },
                                None => return None
                            }
                        },
                        None => return None
                    }
                },
                Oper::Sub => {
                    match left {
                        Some(l) => {
                            match right {
                                Some(r) => {
                                    new_rpn_expr.push(Oper::Operand(l - r));
                                    left = None;
                                    right = None;
                                    counter += 1;
                                    was_operation = true;
                                },
                                None => return None
                            }
                        },
                        None => return None
                    }
                },
                Oper::Mult => {
                    match left {
                        Some(l) => {
                            match right {
                                Some(r) => {
                                    new_rpn_expr.push(Oper::Operand(l * r));
                                    left = None;
                                    right = None;
                                    counter += 1;
                                    was_operation = true;
                                },
                                None => return None
                            }
                        },
                        None => return None
                    }
                },
                Oper::Div => {
                    match left {
                        Some(l) => {
                            match right {
                                Some(r) => {
                                    new_rpn_expr.push(Oper::Operand(l / r));
                                    left = None;
                                    right = None;
                                    counter += 1;
                                    was_operation = true;
                                },
                                None => return None
                            }
                        },
                        None => return None
                    }
                }
                _ => return None
            }
        }
    }
    
    match left {
        Some(n) if right == None => { 
            new_rpn_expr.push(Oper::Operand(n));
            counter += 1;
        }
        _ => {}
    }

    if counter > 1 {
        return recursive_calculate(&new_rpn_expr);
    } 
    else {
        let result = new_rpn_expr.pop();
        match result {
            Some(operand) => {
                match operand {
                    Oper::Operand(number) => {
                        return Some(number);
                    },
                    _ => return None
                }
            },
            _ => return None
        }
    }
}

#[cfg(test)]
mod tests {

    // ****************************************************************************************************
    //
    //
    //is_math_expr tests **********************************************************************************
    #[test]
    fn is_math_expr_looks_like_math_expr_without_whitespaces() {
        use super::*;

        for message in ["2+2", "3*3", "4/4", "5-5", "1**1", "6//6", "7%7", ")8(8", "9^9"] {
            assert!(is_math_expr(&String::from(message)));
        }
    }

    #[test]
    fn is_math_expr_looks_like_math_expr_with_whitespaces() {
        use super::*;

        for message in ["2 + 2", "3 * 3", "4 /4", "5- 5", "1* *1", "6    //6", "7%   7", ") 8(    8", "9^ 9"] {
            assert!(is_math_expr(&String::from(message)));
        }
    }

    #[test]
    fn is_math_expr_definitely_not_math_expr() {
        use super::*;

        for message in ["2 + 2f", "3 kk* 3", "4 !/4", "5- ?5", "1* nana*1", "word", "another word", ""] {
            assert_eq!(is_math_expr(&String::from(message)), false);
        }
    }

    // ****************************************************************************************************
    //
    //
    //convert tests ***************************************************************************************
    #[test]
    fn convert_numbers() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        assert_eq!(convert(&String::from("2387")), res)
    }

    #[test]
    fn convert_numbers_with_whitespaces() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        assert_eq!(convert(&String::from("2 3 87")), res)
    }

    #[test]
    fn convert_numbers_with_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_two_operations_in_a_row() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Add);
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 ++ 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_extra_operation() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5+")), res)
    }

    #[test]
    fn convert_numbers_with_two_pluses_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_minus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Sub);
        assert_eq!(convert(&String::from("2 3 87 - 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_minus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Sub);
        assert_eq!(convert(&String::from("2 3 87 + 49 5- 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_minus_and_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Sub);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 - 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 * 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Mult);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_mult_and_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Mult);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 * 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_div_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Div);
        assert_eq!(convert(&String::from("2 3 87 / 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_div_and_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Div);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 / 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_div_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Div);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5/ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_div_and_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Div);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 / 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_rem_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Rem);
        assert_eq!(convert(&String::from("2 3 87 % 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_rem_and_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Rem);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 % 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_rem_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Rem);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5% 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_rem_and_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Rem);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 % 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_exp_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Exp);
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5")), res)
    }

    #[test]
    fn convert_numbers_with_exp_and_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Exp);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5* 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_mult_and_exp_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Exp);
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 * 49 5^ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_exp_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Exp);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5^ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_exp_and_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Exp);
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5+ 43 0 21")), res)
    }

    #[test]
    fn convert_numbers_with_plus_and_mult_and_exp_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Operand(1509.0));
        res.push(Oper::Exp);
        res.push(Oper::Mult);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5* 43 0 21 ^15 09")), res)
    }

    // ****************************************************************************************************
    //
    //
    //recursive_calculate tests *************************************************************************************
    #[test]
    fn calculate_empty_expr() {
        use super::*;
        
        let rpn: Vec<Oper> = Vec::new();
        assert_eq!(recursive_calculate(&rpn), None);
    }

    #[test]
    fn calculate_operand() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        assert_eq!(recursive_calculate(&rpn), Some(189.0));
    }

    #[test]
    fn calculate_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        assert_eq!(recursive_calculate(&rpn), None);
    }

    #[test]
    fn calculate_plus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Some(719.0));
    }

    #[test]
    fn calculate_plus_too_few_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), None);
    }

    #[test]
    fn calculate_plus_too_many_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), None);
    }

    #[test]
    fn calculate_plus_too_many_operations() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), None);
    }

    #[test]
    fn calculate_plus_and_plus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Some(1044.0));
    }

    #[test]
    fn calculate_minus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Some(341.0));
    }
    
    #[test]
    fn calculate_minus_negative_result_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Some(-341.0));
    }

    #[test]
    fn calculate_plus_and_minus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Some(394.0));
    }

    #[test]
    fn calculate_plus_and_minus_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Some(-666.0));
    }

    #[test]
    fn calculate_mult_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Some(100170.0));
    }
    
    #[test]
    fn calculate_mult_one_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Some(-100170.0));
    }

    #[test]
    fn calculate_mult_two_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-189.0));
        rpn.push(Oper::Operand(-530.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Some(100170.0));
    }

    #[test]
    fn calculate_plus_and_mult_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Some(233675.0));
    }

    #[test]
    fn calculate_plus_and_mult_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Some(161595.0));
    }

    #[test]
    fn calculate_div_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Some(5.0));
    }
    
    #[test]
    fn calculate_div_one_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Some(-5.0));
    }

    #[test]
    fn calculate_div_two_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-530.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Some(5.0));
    }

    #[test]
    fn calculate_plus_and_div_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(513.0));
        rpn.push(Oper::Operand(17.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Some(5.0));
    }

    #[test]
    fn calculate_plus_and_div_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(98.0));
        rpn.push(Oper::Operand(8.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Some(5.0));
    }
}
