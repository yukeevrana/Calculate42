#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorType {
    NotMathExpr,
    BracketsNotAgreed,
    OperandNotNumber,
    MissedOperation,
    MissedOperand,
    UnknownError
}

#[derive(Debug, PartialEq)]
pub struct Error {
    error_type: ErrorType,
    details: String
}

impl Error {
    pub fn new(error_type: ErrorType) -> Error {
        Error {
            details: match error_type {
                ErrorType::NotMathExpr => { String::from("Input is not a mathematical expression.") },
                ErrorType::BracketsNotAgreed => { String::from("Brackets in the expression are not agreed.") },
                ErrorType::OperandNotNumber => { String::from("One of operands is not a correct number.") },
                ErrorType::MissedOperation => { String::from("Missed operation.") },
                ErrorType::MissedOperand => { String::from("Missed operand.") },
                ErrorType::UnknownError => { String::from("Unknown error.") }   
            },
            error_type: error_type
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}