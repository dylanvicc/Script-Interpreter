use token::Token;
use node::Node;

mod token;
mod node;

fn main() {

    let input = "wait(3) 6 * (2 + 4) wait(2) (2 + 1)";
    let tokens = lex(input);

    let tree = parse(&tokens);
    evaluate(&tree);
}

fn lex(characters: &str) -> Vec<Token> {

    let mut tokens = Vec::new();

    let mut number_buffer = String::new();
    let mut keyword_buffer = String::new();

    for character in characters.chars() {
        match character {
            '+' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                tokens.push(Token::Plus);
            }
            '-' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                tokens.push(Token::Minus);
            }
            '*' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                tokens.push(Token::Multiply);
            }
            '/' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                tokens.push(Token::Divide);
            }
            '(' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);

                if !keyword_buffer.is_empty() {

                    if keyword_buffer == "wait" {
                        tokens.push(Token::WaitKeyword);
                    }

                    keyword_buffer.clear();
                }

                tokens.push(Token::LeftParenthesis);
            }
            ')' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                tokens.push(Token::RightParenthesis);
            }
            '0'..='9' => {
                number_buffer.push(character);
            }
            'a'..='z' | 'A'..='Z' => {
                keyword_buffer.push(character);
            }
            ' ' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);

                if !keyword_buffer.is_empty() {

                    if keyword_buffer == "wait" {
                        tokens.push(Token::WaitKeyword);
                    }

                    keyword_buffer.clear();
                }
            }
            _ => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                
                if !keyword_buffer.is_empty() {
                    
                    if keyword_buffer == "wait" {
                        tokens.push(Token::WaitKeyword);
                    }

                    keyword_buffer.clear();
                }
            }
        }
    }

    flush_number_buffer(&mut number_buffer, &mut tokens);

    tokens
}

fn flush_number_buffer(number_buffer: &mut String, tokens: &mut Vec<Token>) {
    
    if !number_buffer.is_empty() {
        
        let number = number_buffer.parse::<i32>().unwrap();
        
        tokens.push(Token::Number(number));       
        number_buffer.clear();
    }
}

fn parse(tokens: &[Token]) -> Node {
    
    let mut it = tokens.iter().peekable();
    let mut sequence = Vec::new();

    while it.peek().is_some() {
        sequence.push(parse_expression(&mut it));
    }

    if sequence.len() == 1 {
        sequence.pop().unwrap()
    } else {
        Node::Sequence(sequence)
    }
}

fn parse_expression<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Node
where
    T: Iterator<Item = &'a Token>,
{
    let mut node = parse_term(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            
            Token::Plus | Token::Minus => {

                let operator = tokens.next().unwrap();
                let right = parse_term(tokens); 

                node = match operator {
                    Token::Plus => Node::Add(Box::new(node), Box::new(right)),
                    Token::Minus => Node::Subtract(Box::new(node), Box::new(right)),
                    _ => unreachable!(),
                };
            }
            _ => break,
        }
    }

    node
}

fn parse_term<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Node
where
    T: Iterator<Item = &'a Token>,
{
    let mut node = parse_factor(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            
            Token::Multiply | Token::Divide => {

                let operator = tokens.next().unwrap();
                let right = parse_factor(tokens);
                
                node = match operator {
                    Token::Multiply => Node::Multiply(Box::new(node), Box::new(right)),
                    Token::Divide => Node::Divide(Box::new(node), Box::new(right)),
                    _ => unreachable!(),
                };
            }
            _ => break,
        }
    }

    node
}

fn parse_factor<'a, T>(tokens: &mut std::iter::Peekable<T>) -> Node
where
    T: Iterator<Item = &'a Token>,
{
    if let Some(token) = tokens.next() {
        match token {
            
            Token::Number(value) => Node::Number(*value),
            
            Token::WaitKeyword => {
              
                if let Some(Token::LeftParenthesis) = tokens.next() {

                    if let Some(Token::Number(value)) = tokens.next() {
      
                        if let Some(Token::RightParenthesis) = tokens.next() {
                            return Node::Wait(*value);
                        } else {
                            panic!("Missing closing parenthesis for 'wait'.");
                        }
                    } else {
                        panic!("Expected a number inside 'wait()'.");
                    }
                } else {
                    panic!("Expected '(' after 'wait'.");
                }
            },
            Token::LeftParenthesis => {

                let expression = parse_expression(tokens);

                if let Some(Token::RightParenthesis) = tokens.next() {
                    expression
                } else {
                    panic!("Missing closing paranthesis.");
                }
            }
            _ => panic!("Unexpected token encountered '{:?}'.", token),
        }
    } else {
        panic!("Unexpected end reached.");
    }
}

fn evaluate(node: &Node) -> i32 {
    match node {
        Node::Number(value) => *value,
        Node::Add(left, right) => evaluate(left) + evaluate(right),
        Node::Subtract(left, right) => evaluate(left) - evaluate(right),
        Node::Multiply(left, right) => evaluate(left) * evaluate(right),
        Node::Divide(left, right) => {
            
            let divisor = evaluate(right);
            
            if divisor == 0 {
                panic!("Division by zero encountered.");
            }
            
            evaluate(left) / divisor
        }
        Node::Wait(value) => {
            std::thread::sleep(std::time::Duration::from_secs(*value as u64));
            0
        }
        Node::Sequence(nodes) => {
            
            let mut last = 0;
            
            for it in nodes {
                last = evaluate(it);
            }
            
            last
        }
    }
}
