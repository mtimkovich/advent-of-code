use regex::{Captures, Regex};

#[derive(Debug, PartialEq)]
enum TokenType {
    OPERATOR,
    PAREN,
    NUMBER,
}

#[derive(Debug)]
struct Token<'a> {
    tok_type: TokenType,
    raw: &'a str,
    val: u32,
}

impl<'a> Token<'a> {
    fn new(caps: Captures<'a>) -> Self {
        let raw = caps.get(0).unwrap().as_str();
        if ["*", "+"].contains(&raw) {
            return Token {
                tok_type: TokenType::OPERATOR,
                raw: raw,
                val: 0,
            };
        } else if ["(", ")"].contains(&raw) {
            return Token {
                tok_type: TokenType::PAREN,
                raw: raw,
                val: 0,
            };
        } else {
            return Token {
                tok_type: TokenType::NUMBER,
                raw: raw,
                val: raw.parse::<u32>().unwrap(),
            };
        }
    }
}

fn tokenize(line: &String) -> Vec<Token> {
    let re = Regex::new(r"(\d+|[+*\(\)])").unwrap();
    let mut tokens = Vec::new();
    for caps in re.captures_iter(line) {
        tokens.push(Token::new(caps));
    }

    tokens
}

fn reduce(stack: &mut Vec<u32>, op: &Token, operands: &mut Vec<&Token>) {
    let a;
    let mut result = stack.last_mut().unwrap();

    // TODO: Case for operands.len() == 0, 1, and 2.
    // 0 operands, combine stacks
    // 1 operand combine to stack
    // 2 result is now stack
    if operands.len() == 0 {
        a = stack.pop().unwrap();
        result = stack.last_mut().unwrap();
    } else {
        a = operands.pop().unwrap().val;
        if let Some(b) = operands.pop() {
            *result = b.val;
        }
    }


    // println!("{} {} {}", *result, op.raw, a.val);

    if op.raw == "+" {
        *result += a;
    } else if op.raw == "*" {
        *result *= a;
    }
}

fn parse(tokens: &Vec<Token>) -> u32 {
    let mut stack = vec![0];
    let mut operands = Vec::new();
    let mut ops: Vec<&Token> = Vec::new();
    for token in tokens {
        // println!("{}", token.raw);
        // println!("{:?}", ops);
        // println!("{:?}", operands);

        match token.tok_type {
            TokenType::NUMBER => operands.push(token),
            TokenType::OPERATOR => {
                if let Some(op) = ops.pop() {
                    if op.tok_type == TokenType::PAREN {
                        ops.push(op);
                    } else {
                        reduce(&mut stack, op, &mut operands);
                    }
                }

                ops.push(token);
            }
            TokenType::PAREN => {
                if token.raw == "(" {
                    ops.push(token);
                    stack.push(0);
                } else {
                    loop {
                        let op = ops.pop().unwrap();
                        if op.raw == "(" {
                            break;
                        }

                        reduce(&mut stack, op, &mut operands);
                    }
                }
            }
        };
    }

    if ops.len() > 0 {
        reduce(
            &mut stack,
            ops.pop().unwrap(),
            &mut operands,
        );
    }

    assert_eq!(stack.len(), 1);
    let res = *stack.first().unwrap();
    res
}

pub fn part1(lines: &Vec<String>) {
    for line in lines {
        let tokens = tokenize(line);
        let res = parse(&tokens);
        println!("{}", res);
    }
}
