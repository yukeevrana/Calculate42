use regex;

// TODO: recursive enum with operations that contains other operations (use Box<T>)
#[derive(Clone, Copy, Debug, PartialEq)]
enum Oper {
    Add,
    Sub,
    Mult,
    Div,
    Rem,
    Exp,
    Bracket,
    // TODO: unary negation (right-associative) with priority like Exp
    Operand(f64)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CalculateErrorType {
    NotMathExpr,
    BracketsNotAgreed,
    OperandNotNumber,
    MissedOperation,
    MissedOperand,
    UnknownError
}

#[derive(Debug, PartialEq)]
pub struct CalculateError {
    error_type: CalculateErrorType,
    details: String
}

impl CalculateError {
    fn new(error_type: CalculateErrorType) -> CalculateError {
        CalculateError {
            details: match error_type {
                CalculateErrorType::NotMathExpr => { String::from("Input is not a mathematical expression.") },
                CalculateErrorType::BracketsNotAgreed => { String::from("Brackets in the expression are not agreed.") },
                CalculateErrorType::OperandNotNumber => { String::from("One of operands is not a correct number.") },
                CalculateErrorType::MissedOperation => { String::from("Missed operation.") },
                CalculateErrorType::MissedOperand => { String::from("Missed operand.") },
                CalculateErrorType::UnknownError => { String::from("Unknown error.") }   
            },
            error_type: error_type
        }
    }
}

impl std::fmt::Display for CalculateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CalculateError {
    fn description(&self) -> &str {
        &self.details
    }
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

pub fn try_calculate(message: &String) -> Result<f64, CalculateError> {
    if !is_math_expr(message) { return Err(CalculateError::new(CalculateErrorType::NotMathExpr)) }
    if !are_brackets_agreed(message) { return Err(CalculateError::new(CalculateErrorType::BracketsNotAgreed)) }

    recursive_calculate(&convert(message)?)
}

/// Checks if string is a valid math expression 
fn is_math_expr(message: &String) -> bool {
    let re = regex::Regex::new(r"^[\d\s\+\-\*/%\(\)\^\.,]+$").unwrap(); // Numbers, whitespaces, +, -, *, /, %, (, )

    re.is_match(message.as_str())
}

/// Checks if the string has the correct amount and order of brackets
fn are_brackets_agreed(message: &String) -> bool {
    let mut left_counter: u32 = 0;
    let mut right_counter: u32 = 0;

    for ch in message.chars() {
        match ch {
            '(' => left_counter += 1,
            ')' => right_counter += 1,
            _ => {}
        }
        if right_counter > left_counter { return false }
    }
    
    if left_counter != right_counter { false }
    else { true }
}

fn try_push_operand(operand: &mut String, stack: &mut Vec<Oper>) -> bool {

    if operand.chars().count() <= 15 { 
        match operand.parse() {
            Ok(n) => { 
                stack.push(Oper::Operand(n));
                operand.clear();
                return true;
            },
            _ => {}
        }
    }
    false
}

/// Converts a string with a *valid* (but not necessarily correct) math expression 
/// to a stack with an expression in RPN. Tests will show in detail.
fn convert(math_expr: &String) -> Result<Vec<Oper>, CalculateError> {
    let mut result: Vec<Oper> = Vec::new();
    let mut temp: Vec<Oper> = Vec::new();
    let mut operand = String::new();

    for current_ch in math_expr.replace(" ", "").replace(",", ".").chars() {
        match current_ch {
            operation_symbol if 
                operation_symbol == '+' || operation_symbol == '-' || operation_symbol == '*' || 
                operation_symbol == '/' || operation_symbol == '%' || operation_symbol == '^' || 
                operation_symbol == '(' => {

                // If found an operation symbol, the previous number has ended, so we will add it to result
                if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalculateError::new(CalculateErrorType::OperandNotNumber)) };

                let current_operation = match operation_symbol {
                    '+' => Oper::Add,
                    '-' => Oper::Sub,
                    '*' => Oper::Mult,
                    '%' => Oper::Rem,
                    '^' => Oper::Exp,
                    '(' => Oper::Bracket,
                    _ => Oper::Div // We don't need to check, the main check in the 'if' above
                };

                if current_operation != Oper::Bracket {
                    loop {
                        match temp.last() {
                            Some(last_operation) if last_operation.get_priority() >= current_operation.get_priority() => {
                                result.push(*last_operation);
                                temp.pop();
                            }
                            _ => { break; }
                        }
                    }
                }

                temp.push(current_operation);
            },
            ')' => {                
                // If found a bracket, the previous number has ended, so we will add it to result
                if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalculateError::new(CalculateErrorType::OperandNotNumber)) };

                loop {
                    match temp.last() {
                        Some(not_bracket) if not_bracket != &Oper::Bracket => {
                            result.push(*not_bracket);
                            temp.pop();
                        },
                        Some(bracket) if bracket == &Oper::Bracket => {
                            temp.pop();
                            break;
                        }
                        _ => { break; }
                    }
                }
            }
            number => {
                operand.push(number); // If a char is not an operation symbol, it is a number, this fn doesn't check
            }
        }
    }
    
    // Don't forget the last number
    if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalculateError::new(CalculateErrorType::OperandNotNumber)) };

    // Don't forget operations on the stack
    temp.reverse();
    for oper in temp {
        result.push(oper);
    }

    Ok(result)
}

fn recursive_calculate(rpn_expr: &Vec<Oper>) -> Result<f64, CalculateError> {
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
                _ => {
                    let left_number: f64;
                    match left {
                        Some(n) => left_number = n,
                        None => return Err(CalculateError::new(CalculateErrorType::MissedOperand))
                    }
                    let right_number: f64;
                    match right {
                        Some(n) => right_number = n,
                        None => return Err(CalculateError::new(CalculateErrorType::MissedOperand))
                    }

                    left = None;
                    right = None;
                    counter += 1;
                    was_operation = true;
                    let result: f64;

                    result = match oper {
                        Oper::Add => left_number + right_number,
                        Oper::Sub => left_number - right_number,
                        Oper::Mult => left_number * right_number,
                        Oper::Div => left_number / right_number,
                        Oper::Rem => left_number % right_number,
                        Oper::Exp => left_number.powf(right_number),
                        _ => return Err(CalculateError::new(CalculateErrorType::UnknownError))
                    };

                    new_rpn_expr.push(Oper::Operand(result));
                }
            }
        }
    }
    
    match left {
        Some(n) if right == None => { 
            new_rpn_expr.push(Oper::Operand(n));
            counter += 1;
        }
        Some(_) if right != None => {
            return Err(CalculateError::new(CalculateErrorType::MissedOperation))
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
                        return Ok(number);
                    },
                    _ => return Err(CalculateError::new(CalculateErrorType::UnknownError))
                }
            },
            _ => return Err(CalculateError::new(CalculateErrorType::NotMathExpr))
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

        for message in ["2.0+2,0", "3*3", "4/4", "5-5", "1**1", "6//6", "7%7", ")8(8", "9^9"] {
            assert!(is_math_expr(&String::from(message)));
        }
    }

    #[test]
    fn is_math_expr_looks_like_math_expr_with_whitespaces() {
        use super::*;

        for message in ["2.0 + 2, 2", "3 * 3", "4 /4", "5- 5", "1* *1", "6    //6", "7%   7", ") 8(    8", "9^ 9"] {
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
    //are_brackets_agreed tests ****************************************************************************
    #[test]
    fn is_brackets_agreed_correct() {
        use super::*;

        for message in ["(2 + 2f)", "3 kk* (3)", "((4) !/(4))", "(5)- ?(5)", "1* nana*1", "word", "another word", ""] {
            assert_eq!(are_brackets_agreed(&String::from(message)), true);
        }
    }

    #[test]
    fn is_brackets_agreed_more_left() {
        use super::*;

        for message in ["((2 + 2f)", "(3 kk* (3)", "(((4) !/(4))", "((5)- ?(5)", "1* nana*(1", "(word", "another (word", "("] {
            assert_eq!(are_brackets_agreed(&String::from(message)), false);
        }
    }
    
    #[test]
    fn is_brackets_agreed_more_right() {
        use super::*;

        for message in ["(2) + 2f)", "3) kk* (3)", "((4) !/(4)))", "(5))- ?(5)", ")1* nana*1", "word)", "another) word", ")"] {
            assert_eq!(are_brackets_agreed(&String::from(message)), false);
        }
    }

    #[test]
    fn is_brackets_agreed_incorrect() {
        use super::*;

        for message in [")2 + 2f(", "3 kk* )3(", "((4) !/)4)(", ")5(- ?(5)"] {
            assert_eq!(are_brackets_agreed(&String::from(message)), false);
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
        assert_eq!(convert(&String::from("2387")), Ok(res))
    }

    #[test]
    fn convert_non_integer_numbers_with_dots() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.2));
        assert_eq!(convert(&String::from("2387.2")), Ok(res))
    }

    #[test]
    fn convert_non_integer_numbers_with_commas() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.2));
        assert_eq!(convert(&String::from("2387,2")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_whitespaces() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        assert_eq!(convert(&String::from("2 3 87")), Ok(res))
    }

    #[test]
    fn convert_non_integer_numbers_with_dots_and_whitespaces() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.2));
        assert_eq!(convert(&String::from("23 8 7. 2")), Ok(res))
    }

    #[test]
    fn convert_non_integer_numbers_with_commas_and_whitespaces() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.2));
        assert_eq!(convert(&String::from("2 387 , 2")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_plus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_plus_two_operations_in_a_row() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Add);
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 ++ 49 5")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_plus_extra_operation() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Add);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + 49 5+")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5+ 43 0 21")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_minus_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Sub);
        assert_eq!(convert(&String::from("2 3 87 - 49 5")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5- 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 - 49 5+ 43 0 21")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_mult_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Mult);
        assert_eq!(convert(&String::from("2 3 87 * 49 5")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5* 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 * 49 5+ 43 0 21")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_div_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Div);
        assert_eq!(convert(&String::from("2 3 87 / 49 5")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 / 49 5* 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5/ 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 / 49 5+ 43 0 21")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_rem_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Rem);
        assert_eq!(convert(&String::from("2 3 87 % 49 5")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 % 49 5* 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5% 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 % 49 5+ 43 0 21")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_exp_correct() {
        use super::*;
        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Exp);
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5* 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 * 49 5^ 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5^ 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 ^ 49 5+ 43 0 21")), Ok(res))
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
        assert_eq!(convert(&String::from("2 3 87 + 49 5* 43 0 21 ^15 09")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_plus_sub_and_brackets_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Sub);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + (49 5- 43 0 21)")), Ok(res))
    }

    #[test]
    fn convert_numbers_with_plus_sub_and_many_brackets_correct() {
        use super::*;

        let mut res: Vec<Oper> = Vec::new();
        res.push(Oper::Operand(2387.0));
        res.push(Oper::Operand(495.0));
        res.push(Oper::Operand(43021.0));
        res.push(Oper::Operand(534.0));
        res.push(Oper::Add);
        res.push(Oper::Sub);
        res.push(Oper::Add);
        assert_eq!(convert(&String::from("2 3 87 + (49 5- (43 0 21 +534))")), Ok(res))
    }
    
    #[test]
    fn convert_operand_before_an_operation_is_not_a_number() {
        use super::*;
        assert_eq!(convert(&String::from("2 3 ..87 + 49 5")), Err(CalculateError::new(CalculateErrorType::OperandNotNumber)))
    }
        
    #[test]
    fn convert_operand_before_a_bracket_is_not_a_number() {
        use super::*;
        assert_eq!(convert(&String::from("(2 3 87 + 49.. 5)")), Err(CalculateError::new(CalculateErrorType::OperandNotNumber)))
    }
     
    #[test]
    fn convert_last_operand_is_not_a_number() {
        use super::*;
        assert_eq!(convert(&String::from("2 3 87 + 49.. 5")), Err(CalculateError::new(CalculateErrorType::OperandNotNumber)))
    }

    // ****************************************************************************************************
    //
    //
    //recursive_calculate tests ***************************************************************************
    #[test]
    fn calculate_empty_expr() {
        use super::*;
        
        let rpn: Vec<Oper> = Vec::new();
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::NotMathExpr)));
    }

    #[test]
    fn calculate_operand() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        assert_eq!(recursive_calculate(&rpn), Ok(189.0));
    }

    #[test]
    fn calculate_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::MissedOperation)));
    }

    #[test]
    fn calculate_plus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Ok(719.0));
    }

    #[test]
    fn calculate_plus_too_few_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::MissedOperand)));
    }

    #[test]
    fn calculate_plus() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::MissedOperand)));
    }

    #[test]
    fn calculate_plus_too_many_operands() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(325.0));
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::MissedOperation)));
    }

    #[test]
    fn calculate_plus_too_many_operations() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Add);
        assert_eq!(recursive_calculate(&rpn), Err(CalculateError::new(CalculateErrorType::MissedOperand)));
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
        assert_eq!(recursive_calculate(&rpn), Ok(1044.0));
    }

    #[test]
    fn calculate_minus_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Ok(341.0));
    }
    
    #[test]
    fn calculate_minus_negative_result_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Sub);
        assert_eq!(recursive_calculate(&rpn), Ok(-341.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(394.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(-666.0));
    }

    #[test]
    fn calculate_mult_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(189.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Ok(100170.0));
    }
    
    #[test]
    fn calculate_mult_one_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-189.0));
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Ok(-100170.0));
    }

    #[test]
    fn calculate_mult_two_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-189.0));
        rpn.push(Oper::Operand(-530.0));
        rpn.push(Oper::Mult);
        assert_eq!(recursive_calculate(&rpn), Ok(100170.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(233675.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(161595.0));
    }

    #[test]
    fn calculate_div_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Ok(5.0));
    }
    
    #[test]
    fn calculate_div_one_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(530.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Ok(-5.0));
    }

    #[test]
    fn calculate_div_two_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-530.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Div);
        assert_eq!(recursive_calculate(&rpn), Ok(5.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(5.0));
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
        assert_eq!(recursive_calculate(&rpn), Ok(5.0));
    }
    
    #[test]
    fn calculate_rem_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(533.0));
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(3.0));
    }
    
    #[test]
    fn calculate_rem_left_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-533.0));
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(-3.0));
    }
        
    #[test]
    fn calculate_rem_right_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(533.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(3.0));
    }

    #[test]
    fn calculate_rem_two_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-533.0));
        rpn.push(Oper::Operand(-106.0));
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(-3.0));
    }

    #[test]
    fn calculate_plus_and_rem_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(513.0));
        rpn.push(Oper::Operand(20.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Operand(106.0));
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(3.0));
    }

    #[test]
    fn calculate_plus_and_rem_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(533.0));
        rpn.push(Oper::Operand(98.0));
        rpn.push(Oper::Operand(8.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Rem);
        assert_eq!(recursive_calculate(&rpn), Ok(3.0));
    }

    #[test]
    fn calculate_exp_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(5.0));
        rpn.push(Oper::Operand(3.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(125.0));
    }
    
    #[test]
    fn calculate_exp_left_negative_right_odd_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-5.0));
        rpn.push(Oper::Operand(3.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(-125.0));
    }
        
    #[test]
    fn calculate_exp_left_negative_right_even_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-5.0));
        rpn.push(Oper::Operand(4.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(625.0));
    }

    #[test]
    fn calculate_exp_right_negative_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(5.0));
        rpn.push(Oper::Operand(-2.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(1.0 / 25.0));
    }

    #[test]
    fn calculate_div_two_negative_right_even_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-5.0));
        rpn.push(Oper::Operand(-2.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(1.0 / 25.0));
    }
    
    #[test]
    fn calculate_div_two_negative_right_odd_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(-5.0));
        rpn.push(Oper::Operand(-3.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(-1.0 / 125.0));
    }

    #[test]
    fn calculate_plus_and_exp_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(4.0));
        rpn.push(Oper::Operand(1.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Operand(3.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(125.0));
    }

    #[test]
    fn calculate_plus_and_exp_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(5.0));
        rpn.push(Oper::Operand(2.0));
        rpn.push(Oper::Operand(1.0));
        rpn.push(Oper::Add);
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(125.0));
    }
    
    #[test]
    fn calculate_mult_and_exp_with_brackets_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(4.0));
        rpn.push(Oper::Operand(2.0));
        rpn.push(Oper::Mult);
        rpn.push(Oper::Operand(2.0));
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(64.0));
    }

    #[test]
    fn calculate_mult_and_exp_correct() {
        use super::*;
        
        let mut rpn: Vec<Oper> = Vec::new();
        rpn.push(Oper::Operand(2.0));
        rpn.push(Oper::Operand(2.0));
        rpn.push(Oper::Operand(3.0));
        rpn.push(Oper::Mult);
        rpn.push(Oper::Exp);
        assert_eq!(recursive_calculate(&rpn), Ok(64.0));
    }
    
    // ****************************************************************************************************
    //
    //
    //try_calculate tests *********************************************************************************
    #[test]
    fn try_calculate_not_math_expr() {
        use super::*;
        
        let input = String::from("not a math expression");
        assert_eq!(try_calculate(&input), Err(CalculateError::new(CalculateErrorType::NotMathExpr)));
    }

    #[test]
    fn try_calculate_brackets_not_agreed() {
        use super::*;
        
        let input = String::from("(2 + 2(");
        assert_eq!(try_calculate(&input), Err(CalculateError::new(CalculateErrorType::BracketsNotAgreed)));
    }
}
