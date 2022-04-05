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
    assert_eq!(convert(&String::from("2 3 ..87 + 49 5")), Err(Error::new(ErrorType::OperandNotNumber)))
}
    
#[test]
fn convert_operand_before_a_bracket_is_not_a_number() {
    use super::*;
    assert_eq!(convert(&String::from("(2 3 87 + 49.. 5)")), Err(Error::new(ErrorType::OperandNotNumber)))
}
    
#[test]
fn convert_last_operand_is_not_a_number() {
    use super::*;
    assert_eq!(convert(&String::from("2 3 87 + 49.. 5")), Err(Error::new(ErrorType::OperandNotNumber)))
}

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

#[test]
fn calculate_empty_expr() {
    use super::*;
    
    let rpn: Vec<Oper> = Vec::new();
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::NotMathExpr)));
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
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::MissedOperation)));
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
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::MissedOperand)));
}

#[test]
fn calculate_plus() {
    use super::*;
    
    let mut rpn: Vec<Oper> = Vec::new();
    rpn.push(Oper::Add);
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::MissedOperand)));
}

#[test]
fn calculate_plus_too_many_operands() {
    use super::*;
    
    let mut rpn: Vec<Oper> = Vec::new();
    rpn.push(Oper::Operand(189.0));
    rpn.push(Oper::Operand(530.0));
    rpn.push(Oper::Operand(325.0));
    rpn.push(Oper::Add);
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::MissedOperation)));
}

#[test]
fn calculate_plus_too_many_operations() {
    use super::*;
    
    let mut rpn: Vec<Oper> = Vec::new();
    rpn.push(Oper::Operand(189.0));
    rpn.push(Oper::Operand(530.0));
    rpn.push(Oper::Add);
    rpn.push(Oper::Add);
    assert_eq!(recursive_calculate(&rpn), Err(Error::new(ErrorType::MissedOperand)));
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

#[test]
fn try_calculate_not_math_expr() {
    use super::*;
    
    let input = String::from("not a math expression");
    assert_eq!(try_calculate(&input), Err(Error::new(ErrorType::NotMathExpr)));
}

#[test]
fn try_calculate_brackets_not_agreed() {
    use super::*;
    
    let input = String::from("(2 + 2(");
    assert_eq!(try_calculate(&input), Err(Error::new(ErrorType::BracketsNotAgreed)));
}