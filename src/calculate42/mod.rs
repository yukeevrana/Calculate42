use regex;
mod calculate_error;

pub use calculate_error::Error as CalcError;
pub use calculate_error::ErrorType as CalcErrorType;

#[cfg(test)]
mod tests;

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

pub fn try_calculate(message: &String) -> Result<f64, CalcError> {
    if !is_math_expr(message) { return Err(CalcError::new(CalcErrorType::NotMathExpr)) }
    if !are_brackets_agreed(message) { return Err(CalcError::new(CalcErrorType::BracketsNotAgreed)) }

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
fn convert(math_expr: &String) -> Result<Vec<Oper>, CalcError> {
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
                if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalcError::new(CalcErrorType::OperandNotNumber)) };

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
                if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalcError::new(CalcErrorType::OperandNotNumber)) };

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
    if operand != "" && !try_push_operand(&mut operand, &mut result) { return Err(CalcError::new(CalcErrorType::OperandNotNumber)) };

    // Don't forget operations on the stack
    temp.reverse();
    for oper in temp {
        result.push(oper);
    }

    Ok(result)
}

fn recursive_calculate(rpn_expr: &Vec<Oper>) -> Result<f64, CalcError> {
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
                        None => return Err(CalcError::new(CalcErrorType::MissedOperand))
                    }
                    let right_number: f64;
                    match right {
                        Some(n) => right_number = n,
                        None => return Err(CalcError::new(CalcErrorType::MissedOperand))
                    }

                    left = None;
                    right = None;
                    counter += 1;
                    was_operation = true;
                    let result: f64;

                    // TODO: I can't be sure, that the operations went through correctly, if the operands are too big
                    result = match oper {
                        Oper::Add => left_number + right_number,
                        Oper::Sub => left_number - right_number,
                        Oper::Mult => left_number * right_number,
                        Oper::Div => left_number / right_number,
                        Oper::Rem => left_number % right_number,
                        Oper::Exp => left_number.powf(right_number),
                        _ => return Err(CalcError::new(CalcErrorType::UnknownError))
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
            return Err(CalcError::new(CalcErrorType::MissedOperation))
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
                    _ => return Err(CalcError::new(CalcErrorType::UnknownError))
                }
            },
            _ => return Err(CalcError::new(CalcErrorType::NotMathExpr))
        }
    }
}
