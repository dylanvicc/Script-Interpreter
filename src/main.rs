use token::Token;
use node::Node;

mod token;
mod node;

fn main() {

    let tokens = lex("let x = 10; print(x + 5) wait(2) print(x)"); 
    let tree = parse(&tokens);

    evaluate(&tree, &mut std::collections::HashMap::new());
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
                process_keyword(&mut keyword_buffer, &mut tokens);
                tokens.push(Token::LeftParenthesis);
            }
            ')' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                process_keyword(&mut keyword_buffer, &mut tokens);
                tokens.push(Token::RightParenthesis);
            }
            '=' => tokens.push(Token::Equal),
            ';' => {
                flush_number_buffer(&mut number_buffer, &mut tokens);
                process_keyword(&mut keyword_buffer, &mut tokens);
                tokens.push(Token::Semicolon);
            },
            '0'..='9' => number_buffer.push(character),
            'a'..='z' | 'A'..='Z' => keyword_buffer.push(character),
            ' ' => {          
                flush_number_buffer(&mut number_buffer, &mut tokens);
                process_keyword(&mut keyword_buffer, &mut tokens);
            }
            _ => {}
        }
    }

    flush_number_buffer(&mut number_buffer, &mut tokens);
    process_keyword(&mut keyword_buffer, &mut tokens);

    tokens
}

fn process_keyword(keyword_buffer: &mut String, tokens: &mut Vec<Token>) {
    if !keyword_buffer.is_empty() {
        match keyword_buffer.as_str() {
            "wait" => tokens.push(Token::WaitKeyword),
            "print" => tokens.push(Token::PrintKeyword),
            "let" => tokens.push(Token::LetKeyword),
            _ => tokens.push(Token::Identifier(keyword_buffer.clone())),
        }
        keyword_buffer.clear();
    }
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
        if let Some(Token::LetKeyword) = it.peek() {
            it.next();

            if let Some(Token::Identifier(name)) = it.next() {
                if let Some(Token::Equal) = it.next() {
                    
                    let value = parse_expression(&mut it);
                    sequence.push(Node::VariableDeclaration(name.clone(), Box::new(value)));

                    if let Some(Token::Semicolon) = it.next() {
                        continue;
                    } else {
                        panic!("Expected semicolon after variable declaration.");
                    }
                }
            }
        } else {
            sequence.push(parse_expression(&mut it));
        }
    }

    Node::Sequence(sequence)
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
            Token::PrintKeyword => {
                if let Some(Token::LeftParenthesis) = tokens.next() {

                    let expression = parse_expression(tokens); 
                    
                    if let Some(Token::RightParenthesis) = tokens.next() {
                        return Node::Print(Box::new(expression));
                    } else {
                        panic!("Missing closing parenthesis for 'print'.");
                    }
                } else {
                    panic!("Expected '(' after 'print'.");
                }
            },
            Token::Identifier(name) => Node::Variable(name.clone()),
            Token::LeftParenthesis => {
                let expression = parse_expression(tokens);

                if let Some(Token::RightParenthesis) = tokens.next() {
                    expression
                } else {
                    panic!("Missing closing parenthesis.");
                }
            }
            _ => panic!("Unexpected token encountered."),
        }
    } else {
        panic!("Unexpected end reached.");
    }
}

fn evaluate(node: &Node, variables: &mut std::collections::HashMap<String, i32>) -> i32 {
    match node {
        Node::Number(value) => *value,
        Node::Variable(name) => *variables.get(name).expect("Undefined variable."),
        Node::Add(left, right) => evaluate(left, variables) + evaluate(right, variables),
        Node::Subtract(left, right) => evaluate(left, variables) - evaluate(right, variables),
        Node::Multiply(left, right) => evaluate(left, variables) * evaluate(right, variables),
        Node::Print(expression) => {
            let value = evaluate(expression, variables);
            println!("{}", value);
            0
        }
        Node::Divide(left, right) => {
            
            let divisor = evaluate(right, variables);
            
            if divisor == 0 {
                panic!("Division by zero encountered.");
            }
            
            evaluate(left, variables) / divisor
        }
        Node::Wait(value) => {
            std::thread::sleep(std::time::Duration::from_secs(*value as u64));
            0
        }
        Node::Sequence(nodes) => {
            
            let mut last = 0;
            
            for it in nodes {
                last = evaluate(it, variables);
            }
            
            last
        }
        Node::VariableDeclaration(name, value) => {
           
            let _value = evaluate(value, variables);
            variables.insert(name.clone(), _value);
            
            _value
        }
    }
}