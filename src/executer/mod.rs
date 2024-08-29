use crate::scanner::{Token, TokenType};
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
enum ValueType {
    String,
    Number,
    Boolean,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Symbol {
    value: String,
    v_type: ValueType,
    //scope: String,
}

trait IValueType {
    fn value_type(self) -> Option<ValueType>;
}

impl IValueType for &str {
    fn value_type(self) -> Option<ValueType> {
        if self == "TRUE" || self == "FALSE" {
            return Some(ValueType::Boolean);
        }
        if self.parse::<f64>().is_ok() {
            return Some(ValueType::Number);
        }
        return Some(ValueType::String);
    }
}

fn expression_endpoint(tokens: &Vec<Token>, start: usize) -> Option<usize> {
    let mut idx = start;
    while tokens[idx].tk_type != TokenType::Newline && idx < tokens.len() {
        idx += 1;
    }
    if idx > start {
        return Some(idx);
    }
    None
}

fn precedence(operator: &str) -> u8 {
    match TokenType::token_type(operator).unwrap() {
        TokenType::NotOperator => 5,
        TokenType::ExpOperator => 4,
        TokenType::MultOperator | TokenType::DivOperator | TokenType::ModOperator => 3,
        TokenType::AddOperator | TokenType::MinusOperator => 2,
        TokenType::GreaterThan | TokenType::GreaterThanEq | TokenType::LessThan 
            | TokenType::LessThanEq | TokenType::EqualsOperator | TokenType::NotEqualsOperator => 1,
        TokenType::AndOperator | TokenType::OrOperator => 0,
        _ => 0,
    }
}

fn evaluate_infix(symbol_table: &HashMap<&str, Symbol>, infix: &[Token]) -> Result<f64, String> {
    let mut stack: Vec<&Token> = Vec::new();
    let mut postfix: Vec<&str> = Vec::new();
    
    // convert to postfix
    for token in infix {
        let res = token.lexeme.parse::<f64>();
        if token.tk_type == TokenType::Identifier {
            let value = symbol_table.get(token.lexeme);
            if value.is_some() {
                postfix.push(&value.unwrap().value);
            } else {
                let message = std::format!("{} SYMBOL NOT FOUND", token.lexeme);
                return Err(message);
            }
        }
        else if res.is_ok() {
            postfix.push(token.lexeme);
        } else if token.tk_type == TokenType::LParen {
            stack.push(token);
        } else if token.tk_type == TokenType::RParen {
            while stack.len() > 0 && stack.last().unwrap().tk_type != TokenType::LParen {
                postfix.push(stack.pop().unwrap().lexeme);
            }
            stack.pop();
        } else {
            while stack.len() > 0 && precedence(stack.last().unwrap().lexeme) >= precedence(token.lexeme) {
                postfix.push(stack.pop().unwrap().lexeme);
            }
            stack.push(token);
        }
    }
    while stack.len() > 0 {
        postfix.push(stack.pop().unwrap().lexeme);
    }
    let result = evaluate_postfix(&postfix);
    Ok(result)
}

fn evaluate_postfix(postfix: &Vec<&str>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    
    for op in postfix {
        let res = op.parse::<f64>();
        if res.is_ok() {
            stack.push(res.unwrap());
        } else {
            match TokenType::token_type(op) {
                Some(TokenType::TrueLiteral) => {
                    stack.push(1.0);
                }
                Some(TokenType::FalseLiteral) => {
                    stack.push(0.0);
                }
                Some(TokenType::NotOperator) => {
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((!operand1) as i32 as f64);
                }
                Some(TokenType::AndOperator) => {
                    let operand2 = stack.pop().unwrap() as i32 != 0;
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((operand1 && operand2) as i32 as f64);
                }
                Some(TokenType::OrOperator) => {
                    let operand2 = stack.pop().unwrap() as i32 != 0;
                    let operand1 = stack.pop().unwrap() as i32 != 0;
                    stack.push((operand1 || operand2) as i32 as f64);
                }
                Some(TokenType::GreaterThan) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 > operand2) as i32 as f64);
                }
                Some(TokenType::GreaterThanEq) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 >= operand2) as i32 as f64);
                }
                Some(TokenType::LessThan) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 < operand2) as i32 as f64);
                }
                Some(TokenType::LessThanEq) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push((operand1 <= operand2) as i32 as f64);
                }
                Some(TokenType::AddOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1 + operand2);
                }
                Some(TokenType::MinusOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1 - operand2);
                }
                Some(TokenType::MultOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1 * operand2);
                }
                Some(TokenType::DivOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1 / operand2);
                }
                Some(TokenType::ModOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1 % operand2);
                }
                Some(TokenType::ExpOperator) => {
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();
                    stack.push(operand1.powf(operand2));
                }
                _ => {}
            }
        }
    }

    stack.pop().unwrap()
}

pub fn execute(tokens: &Vec<Token>) -> Result<(), String> {
    let mut symbol_table: HashMap<&str, Symbol> = HashMap::new();
    let mut idx: usize = 0;
    let mut skip_scope = false;
    let mut look_elif = false;

    while idx < tokens.len() {
        loop { // 'loop' for breaking the match early
        match tokens[idx].tk_type {
            TokenType::Initialize => {
                if skip_scope { break }
                idx += 1;
                if tokens[idx].tk_type == TokenType::Identifier {
                    let id = &tokens[idx];
                    idx += 1;
                    if tokens[idx].tk_type == TokenType::Assign {
                        idx += 1;
                        let end = expression_endpoint(tokens, idx).unwrap();
                        if end - idx > 1 {
                            let expression_tokens = &tokens[idx..end];
                            let value = evaluate_infix(&symbol_table, &expression_tokens);
                            symbol_table.insert(id.lexeme, Symbol {
                                value: value.unwrap().to_string(), 
                                v_type: ValueType::Number,
                            });
                        } else {
                            let value = tokens[idx].lexeme;
                            let value_type = value.value_type();
                            symbol_table.insert(id.lexeme, Symbol {
                                value: value.to_string(), 
                                v_type: value_type.unwrap(),
                            });
                        }
                        idx = end;
                    } else {
                        let error_message = std::format!("EXPECTED ASSIGN OPERATOR, FOUND '{}'", tokens[idx].lexeme);
                        return Err(error_message);
                    }
                } else {
                    let error_message = std::format!("EXPECTED IDENTIFIER SYMBOL, FOUND '{}'", tokens[idx].lexeme);
                    return Err(error_message);
                }
            },
            TokenType::Identifier => {
                if skip_scope { break }
                let symbol = tokens[idx].lexeme;
                let id = symbol_table.get(symbol);
                if id.is_none() {
                    let error_message = std::format!("{} SYMBOL NOT FOUND. MAKE SURE TO INITIALIZE WITH 'LET'", symbol);
                    return Err(error_message);
                }
                idx += 1;
                if tokens[idx].tk_type == TokenType::Assign {
                    idx += 1;
                    let end = expression_endpoint(tokens, idx).unwrap();
                    let tokens = &tokens[idx..end];
                    let result = evaluate_infix(&symbol_table, &tokens);
                    match result {
                        Ok(value) => {
                            symbol_table.insert(symbol, Symbol {
                                value: value.to_string(), 
                                v_type: ValueType::Number
                            });
                        },
                        Err(message) => {
                            return Err(message);
                        }
                    }
                    idx = end;
                } else {
                    let error_message = std::format!("EXPECTED ASSIGN OPERATOR, FOUND '{}'", tokens[idx].lexeme);
                    return Err(error_message);
                }
            },
            TokenType::Output => {
                if skip_scope { break }
                idx += 1;
                let tk_print = &tokens[idx];
                if tk_print.tk_type == TokenType::Identifier {
                    let id = symbol_table.get(tk_print.lexeme);
                    if id.is_none() {
                        let error_message = std::format!("{} SYMBOL NOT FOUND. MAKE SURE TO INITIALIZE WITH 'LET'", tk_print.lexeme);
                        return Err(error_message);
                    }
                    // remove quotes if string
                    let value = id.unwrap().value.trim_matches('"');
                    println!("{}", value);
                } else {
                    let value = tk_print.lexeme.trim_matches('"');
                    println!("{}", value);
                }
            },
            TokenType::If => {
                if skip_scope { break }
                idx += 1;
                let end = expression_endpoint(tokens, idx).unwrap();
                let tokens = &tokens[idx..end];
                let result = evaluate_infix(&symbol_table, &tokens);
                match result {
                    Ok(condition) => {
                        if condition == 1.0 {
                            skip_scope = false;
                            look_elif = false;
                        } else {
                            skip_scope = true;
                            look_elif = true;
                        }
                    },
                    Err(message) => {
                        return Err(message);
                    }
                }
            },
            TokenType::Else => {
                idx += 1;
                // will skip ELSE if did IF, otherwise it wont
                skip_scope = !skip_scope;
            },
            TokenType::EndScope => {
                idx += 1;
                skip_scope = false;
            },
            _ => {},
        }
        break;}
        idx += 1;
    }
    //for (symbol, data) in symbol_table {
      //  println!("D: {} = {}", symbol, data.value);
    //}
    Ok(())
}
