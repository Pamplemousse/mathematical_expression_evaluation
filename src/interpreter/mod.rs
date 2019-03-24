use tokenizer::token::Token;

pub fn interpret(tokens: Vec<Token>) -> f32 {
    let mut operands: Vec<f32> = Vec::new();

	for token in tokens {
		match token {
			Token::Operator(operator) => {
                let second_operand = operands.pop().unwrap();
                let first_operand = operands.pop().unwrap();
                let result: f32 = operator.call(first_operand, second_operand);

                operands.push(result);
			}
            Token::Literal(literal) => {
                let operand: f32 = literal.to_string().parse::<f32>().unwrap();
                operands.push(operand);
            }
            _ => panic!("Dev error: The AST should only contains Operators and Literals."),
		}
	}

    return operands.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::parse;
    use tokenizer::tokenize;

    #[test]
    fn interpret_simple_addition() {
        let tokens: Vec<Token> = tokenize("3+2");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 5.0;

       assert_eq!(result, expected_result);
    }

    #[test]
    fn interpret_simple_substraction() {
        let tokens: Vec<Token> = tokenize("3-2");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 1.0;

       assert_eq!(result, expected_result);
    }

    #[test]
    fn interpret_simple_multiplication() {
        let tokens: Vec<Token> = tokenize("3*2");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 6.0;

       assert_eq!(result, expected_result);
    }

    #[test]
    fn interpret_simple_integer_division() {
        let tokens: Vec<Token> = tokenize("6/2");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 3.0;

       assert_eq!(result, expected_result);
    }

    #[test]
    fn interpret_simple_exponentiation() {
        let tokens: Vec<Token> = tokenize("2^3");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 8.0;

       assert_eq!(result, expected_result);
    }

    #[test]
    fn interpret_complex_operation() {
        let tokens: Vec<Token> = tokenize("(2+2)*3^2/3");
        let ast: Vec<Token> = parse(tokens);
        let result: f32 = interpret(ast);

        let expected_result: f32 = 12.0;

       assert_eq!(result, expected_result);
    }
}
