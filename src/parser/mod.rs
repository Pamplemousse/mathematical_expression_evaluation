use tokenizer::token::*;

pub fn parse(tokens: Vec<Token>) -> Vec<Token> {
    let mut ast: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Literal(_) => ast.push(token),
            Token::Operator(operator) => {
                while operator_stack.len() > 0 {
                    let top_token: Token = operator_stack.last().unwrap().clone();

                    // The operator_stack only holds LeftParentheses and Operators
                    match top_token {
                        Token::LeftParenthesis => break,
                        Token::Operator(top_operator) => {
                            if top_operator > operator {
                                operator_stack.pop();
                                ast.push(Token::Operator(top_operator));
                            } else { break; }
                        }
                        _ => ()
                    }
                }

                operator_stack.push(Token::Operator(operator));
            },
            Token::LeftParenthesis => operator_stack.push(token),
            Token::RightParenthesis => {
                while operator_stack.len() > 0 {
                    let top_token: Token = operator_stack.last().unwrap().clone();

                    // The operator_stack only holds LeftParentheses and Operators
                    match top_token {
                        Token::LeftParenthesis => {
                            operator_stack.pop();
                            break;
                        },
                        Token::Operator(top_operator) => {
                            operator_stack.pop();
                            ast.push(Token::Operator(top_operator));
                        }
                        _ => ()
                    }
                }
            },
        }
    }

    operator_stack.reverse();
    ast.append(&mut operator_stack);

    return ast;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::tokenize;
    use tokenizer::token::literal::Literal;
    use tokenizer::token::operator::Operator;

    #[test]
    fn parse_with_empty_list_of_tokens() {
        let tokens: Vec<Token> = tokenize("");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_a_two_literals_multiplication() {
        let tokens: Vec<Token> = tokenize("22*3");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_multiplication_and_addition() {
        let tokens: Vec<Token> = tokenize("22*3+2");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_addition_and_multiplication() {
        let tokens: Vec<Token> = tokenize("22+3*2");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Times),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_multiplication_and_addition_and_multiplication() {
        let tokens: Vec<Token> = tokenize("22*3+2*3");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_addition_and_multiplication_and_addition() {
        let tokens: Vec<Token> = tokenize("22+3*2+3");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_multiplication_and_addition_and_addition() {
        let tokens: Vec<Token> = tokenize("22*3+2+3");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_addition_and_addition_and_multiplication() {
        let tokens: Vec<Token> = tokenize("22+3+2*3");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Times),
            Token::Operator(Operator::Plus),
            Token::Operator(Operator::Plus)
        ].to_vec();

       assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_with_parentheses() {
        let tokens: Vec<Token> = tokenize("(22+3)*2");
        let result: Vec<Token> = parse(tokens);

        let expected_result: Vec<Token> = [
            Token::Literal(Literal::from(String::from("22"))),
            Token::Literal(Literal::from(String::from("3"))),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::from(String::from("2"))),
            Token::Operator(Operator::Times),
        ].to_vec();

       assert_eq!(result, expected_result);
    }
}
