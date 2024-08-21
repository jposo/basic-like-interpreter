use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    Identifier,
    Keyword,
    Initialize,
    Assign,
    Output,
    Separator,
    BeginIf,
    BeginElse,
    BeginElseIf,
    Delimiter,
    EndScope,
    AddOperator,
    MinusOperator,
    MultOperator,
    DivOperator,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    AndOperator,
    OrOperator,
    NotOperator,
    EqualsOperator,
    NotEqualsOperator,
    LParen,
    RParen,
    Operator,
    TrueLiteral,
    FalseLiteral,
    Literal,
    Newline,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ValueType {
    String,
    Number,
    Boolean,
}

struct Identifier<'a> {
    value_type: ValueType,
    name: &'a str,
    value: String,
}

struct Token<'a> {
    tk_type: TokenType,
    lexeme: &'a str,
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Identifier<{:?}, '{:?}' = {}>", self.value_type, self.name, self.value)
    }
}


impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token<{:?}, '{:?}'>", self.tk_type, self.lexeme)
    }
}


fn token_type(value: &str) -> Option<TokenType> {
    if value.parse::<f64>().is_ok() {
        return Some(TokenType::Literal);
    }
    match value {
        "\n" => return Some(TokenType::Newline),
        "LET" => return Some(TokenType::Initialize),
        "END" => return Some(TokenType::EndScope),
        "BE" => return Some(TokenType::Assign),
        "RETURN" => return Some(TokenType::Keyword),
        "IF" => return Some(TokenType::BeginIf),
        "ELSE" => return Some(TokenType::BeginElse),
        "+" => return Some(TokenType::AddOperator),
        "-" => return Some(TokenType::MinusOperator),
        "*" => return Some(TokenType::MultOperator),
        "/" => return Some(TokenType::DivOperator),
        "(" => return Some(TokenType::LParen),
        ")" => return Some(TokenType::RParen),
        "<" => return Some(TokenType::LessThan),
        "<=" => return Some(TokenType::LessThanEq),
        ">" => return Some(TokenType::GreaterThan),
        ">=" => return Some(TokenType::GreaterThanEq),
        "AND" => return Some(TokenType::AndOperator),
        "OR" => return Some(TokenType::OrOperator),
        "NOT" => return Some(TokenType::NotOperator),
        "EQUALS" => return Some(TokenType::EqualsOperator),
        "NOTEQUALS" => return Some(TokenType::NotEqualsOperator),
        "PRINT" => return Some(TokenType::Output),
        "FOR" => return Some(TokenType::Keyword),
        "IN" => return Some(TokenType::Operator),
        "TO" => return Some(TokenType::Operator),
        "FUNCTION" => return Some(TokenType::Keyword),
        "," => return Some(TokenType::Separator),
        "TRUE" => return Some(TokenType::TrueLiteral),
        "FALSE" => return Some(TokenType::FalseLiteral),
        _ => return Some(TokenType::Identifier),
    }
}

pub fn scan(input_string: &String) {
    let mut tokens: Vec<Token> = Vec::new();
    // delimiters seperate keywords, literals, operators, etc..
    let delimiters = vec![' ', ',', '(', ')', '\n', '\t', '\r', '"'];
    // whitespaces get ignore and skipped unless boolean is true
    let whitespaces = vec![' ', '\t', '\r'];
    // for strings
    let mut in_string = false;
    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;
    
    while end_idx < input_string.len() {
        let curr = input_string.chars().nth(end_idx).unwrap();
        let curr_str = &input_string[end_idx..end_idx+1];

        if curr == '"' && !in_string {
            // start of a string
            if start_idx != end_idx {
                // add previous token
                let value = &input_string[start_idx..end_idx];
                tokens.push(Token {
                    tk_type: token_type(value).unwrap(),
                    lexeme: value,
                });
            }
            in_string = true;
            // including the quotation mark as the start of token
            start_idx = end_idx;
        }
        else if curr == '"' && in_string {
            // end of a string
            end_idx += 1;
            let value = &input_string[start_idx..end_idx];
            tokens.push(Token {
                tk_type: TokenType::Literal,
                lexeme: value,
            });
            start_idx = end_idx + 1; // added +1
            in_string = false;
        }
        else if delimiters.contains(&curr) && !in_string {
            // current character is a delimiter
            if start_idx != end_idx {
                // at least a seperation of 1
                let value = &input_string[start_idx..end_idx];
                tokens.push(Token {
                    tk_type: token_type(value).unwrap(),
                    lexeme: value,
                });
            }
            // advancing to the next character
            start_idx = end_idx + 1;
            if !whitespaces.contains(&curr) {
                // current character is not whitespace
                tokens.push(Token {
                    tk_type: token_type(curr_str).unwrap(),
                    lexeme: curr_str,
                });
            }
        }
        // character is not a delimiter, so we continue until one appears
        end_idx += 1;
    }
    let result = evaluate(&tokens);
    if result.is_err() {
        println!("ERROR: {}", result.err().unwrap());
    }
}

trait IsNumeric {
    fn is_numeric(self) -> Option<f64>;
}

impl IsNumeric for &str {
    fn is_numeric(self) -> Option<f64> {
        if self.parse::<f64>().is_ok() {
            return Some(self.parse().unwrap());
        }
        None
    }
}

fn get_data_type(value: &str) -> Option<ValueType> {
    if value == "TRUE" || value == "FALSE" {
        return Some(ValueType::Boolean);
    }
    if value.parse::<f64>().is_ok() {
        return Some(ValueType::Number);
    }
    return Some(ValueType::String);
}

fn precedence(operator: &str) -> u8 {
    match token_type(operator).unwrap() {
        TokenType::NotOperator => 4,
        TokenType::GreaterThan | TokenType::GreaterThanEq | TokenType::LessThan 
            | TokenType::LessThanEq | TokenType::EqualsOperator | TokenType::NotEqualsOperator => 3,
        TokenType::MultOperator | TokenType::DivOperator | TokenType::AndOperator => 2,
        TokenType::AddOperator | TokenType::MinusOperator | TokenType::OrOperator => 1,
        _ => 0,
    }
}

fn evaluate_postfix(postfix: &Vec<&str>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for op in postfix {
        let res = op.is_numeric();
        if res != None {
            stack.push(res.unwrap());
        }
        else {
            let operand2 = stack.pop().unwrap();
            let operand1 = stack.pop().unwrap();

            match token_type(op) {
                Some(TokenType::AddOperator) => {
                    stack.push(operand1 + operand2);
                },
                Some(TokenType::MinusOperator) => {
                    stack.push(operand1 - operand2);
                },
                Some(TokenType::MultOperator) => {
                    stack.push(operand1 * operand2);
                },
                Some(TokenType::DivOperator) => {
                    stack.push(operand1 / operand2);
                },
                _ => {},
            }
        }
    }
    return stack.pop().unwrap();
}

fn evaluate_logic_postfix(postfix: &Vec<&str>) -> bool {
    let mut stack: Vec<f64> = Vec::new();
    
    //println!("{:?}", postfix);
    for op in postfix {
        let res = op.parse::<f64>();
        if res.is_ok() {
            stack.push(res.unwrap());
        }
        else {
            match token_type(op) {
                Some(TokenType::NotOperator) => {
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((!operand1) as i32 as f64);
                },
                Some(TokenType::AndOperator) => {
                    let operand2 = stack.pop().unwrap() as i32 != 0;
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((operand1 && operand2) as i32 as f64);
                },
                Some(TokenType::OrOperator) => {
                    let operand2 = stack.pop().unwrap() as i32 != 0;
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((operand1 || operand2) as i32 as f64);
                },
                Some(TokenType::GreaterThan) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 > operand2) as i32 as f64);
                },
                Some(TokenType::GreaterThanEq) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 >= operand2) as i32 as f64);
                },
                Some(TokenType::LessThan) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 < operand2) as i32 as f64);
                },
                Some(TokenType::LessThanEq) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 <= operand2) as i32 as f64);
                },
                _ => {},
            }
        }
    }
    stack.pop().unwrap() as i32 != 0
}

fn evaluate_infix(expression: &Vec<&str>) -> f64 {
    let mut stack: Vec<&str> = Vec::new();
    let mut postfix: Vec<&str> = Vec::new();
    
    // convert to postfix
    for op in expression {
        if op.is_numeric().is_some() {
            postfix.push(op);
        } else if token_type(op).unwrap() == TokenType::LParen {
            stack.push(op);
        } else if token_type(op).unwrap()  == TokenType::RParen {
            while stack.len() > 0 && token_type(stack[ stack.len() - 1 ]).unwrap() != TokenType::LParen {
                postfix.push(stack.pop().unwrap());
            }
            stack.pop();
        } else {
            while stack.len() > 0 && precedence(stack.last().unwrap()) >= precedence(op) {
                postfix.push(stack.pop().unwrap());
            }
            stack.push(op);
        }
    }
    while stack.len() > 0 {
        postfix.push(stack.pop().unwrap());
    }
    
    let result = evaluate_postfix(&postfix);
    return result;
}

fn evaluate_logic_infix(expression: &Vec<&str>) -> bool {
    let mut stack: Vec<&str> = Vec::new();
    let mut postfix: Vec<&str> = Vec::new();
    
    // convert to postfix
    for op in expression {
        match token_type(op) {
            Some(TokenType::TrueLiteral) | Some(TokenType::FalseLiteral) | Some(TokenType::Literal) => {
                postfix.push(op);
            },
            Some(TokenType::LParen) => {
                stack.push(op);
            },
            Some(TokenType::RParen) => {
                while !stack.is_empty() && token_type(stack.last().unwrap()) != Some(TokenType::LParen) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.pop();
            },
            _ => {
                while !stack.is_empty() && precedence(stack.last().unwrap()) >= precedence(op) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(op);
            }
        }
    }
    while !stack.is_empty() {
        postfix.push(stack.pop().unwrap());
    }
    
    evaluate_logic_postfix(&postfix)
}

struct Scope<'a> {
    name: String,
    symbols: HashMap<&'a str, (String, ValueType)>,
}

fn evaluate(tokens: &Vec<Token>) -> Result<(), String> {
    let mut symbols: HashMap<&str, (String, ValueType)> = HashMap::new();
    //let mut scopes: Vec<Scope> = Vec::new();

    let mut idx: usize = 0;
    while idx < tokens.len() {
        let token = &tokens[idx];
        // initialization
        if token.tk_type == TokenType::Initialize {
            idx += 1;
            if tokens[idx].tk_type == TokenType::Identifier {
                let id = &tokens[idx];
                idx += 1;
                if tokens[idx].tk_type == TokenType::Assign {
                    // eval operation after assign
                    idx += 1;
                    let value = &tokens[idx].lexeme;
                    let value_type = get_data_type(value);
                    if value_type == Some(ValueType::String) {
                        //let value = &value.trim_matches('"');
                    }
                    symbols.insert(id.lexeme, (value.to_string(), value_type.unwrap()));
                } else {
                    let error_message = std::format!("EXPECTED ASSIGN OPERATOR, FOUND '{}'", tokens[idx].lexeme);
                    return Err(error_message);
                }
            } else {
                let error_message = std::format!("EXPECTED IDENTIFIER SYMBOL, FOUND '{}'", tokens[idx].lexeme);
                return Err(error_message);
            }
        }
        // if it doesn't begin with LET, it is an assignment
        else if token.tk_type == TokenType::Identifier {
            let id = symbols.get(token.lexeme);
            if id.is_none() {
                let error_message = std::format!("{} SYMBOL NOT FOUND. MAKE SURE TO INITIALIZE WITH 'LET'", token.lexeme);
                return Err(error_message);
            }
            idx += 1;
            if tokens[idx].tk_type == TokenType::Assign {
                idx += 1;
                let mut infix_stack: Vec<&str> = Vec::new();
                while tokens[idx].tk_type != TokenType::Newline {
                    if tokens[idx].tk_type == TokenType::Identifier {
                        let other_id = symbols.get(tokens[idx].lexeme).unwrap();
                        if other_id.1 != id.unwrap().1 {
                            return Err("CAN NOT OPERATE WITH DIFFERENT VALUE TYPES".to_string());
                        }
                        infix_stack.push(&other_id.0);
                    } else {
                        infix_stack.push(&tokens[idx].lexeme);
                    }
                    idx += 1;
                }
                let result = evaluate_infix(&infix_stack);
                symbols.insert(token.lexeme, (result.to_string(), ValueType::Number));
            } else {
                let error_message = std::format!("EXPECTED ASSIGN OPERATOR, FOUND '{}'", tokens[idx].lexeme);
                return Err(error_message);
            }
        }
        // output
        else if token.tk_type == TokenType::Output {
            idx += 1;
            let tk_print = &tokens[idx];
            if tk_print.tk_type == TokenType::Identifier {
                let id = symbols.get(tk_print.lexeme).unwrap();
                let value_type = get_data_type(&id.0);
                if value_type == Some(ValueType::String) {
                    let value = id.0.trim_matches('"');
                    println!("{}", value);
                } else {
                    println!("{}", id.0);
                }
            } else {
                let value_type = get_data_type(tk_print.lexeme);
                if value_type == Some(ValueType::String) {
                    let value = tk_print.lexeme.trim_matches('"');
                    println!("{}", value);
                }
            }
        }
        // if
        // scope initialting keywords, for, if, while
        else if token.tk_type == TokenType::BeginIf {
            idx += 1;
            let mut logic_expr: Vec<&str> = Vec::new();
            // get tokens to evaluate
            while tokens[idx].tk_type != TokenType::Newline {
                if tokens[idx].tk_type == TokenType::Identifier {
                    let symbol = symbols.get(tokens[idx].lexeme);
                    match symbol {
                        None => {
                            let error_message = std::format!("COULD NOT FIND SYMBOL '{}'", tokens[idx].lexeme);
                            return Err(error_message);
                        },
                        Some(id) => {
                            logic_expr.push(&id.0);
                        },
                    }
                } else {
                    logic_expr.push(&tokens[idx].lexeme);
                }
                idx += 1;
            }
            let result = evaluate_logic_infix(&logic_expr);
            // if result is false skip until end or else
            if result == false {
                while idx < tokens.len() && (tokens[idx].tk_type != TokenType::EndScope || tokens[idx].tk_type != TokenType::BeginElse) {
                    idx += 1;
                }
            }
        }
        idx += 1;
    }
    Ok(())
}
