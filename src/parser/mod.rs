use crate::scanner;
use crate::scanner::{Token, TokenType};
use std::fmt;

#[derive(Debug)]
enum Construct {
    Output,
    Scope,
    Operator,
    Variable,
    Literal,
    Branch,
    Assign,
}

pub struct Node {
    construct: Construct,
    pub value: Option<String>,
    pub children: Vec<Node>,
}

impl<'a> fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node<{:?}, {:?}>", self.construct, self.value)
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
        TokenType::NotOperator => 4,
        TokenType::GreaterThan | TokenType::GreaterThanEq | TokenType::LessThan 
            | TokenType::LessThanEq | TokenType::EqualsOperator | TokenType::NotEqualsOperator => 3,
        TokenType::MultOperator | TokenType::DivOperator | TokenType::AndOperator => 2,
        TokenType::AddOperator | TokenType::MinusOperator | TokenType::OrOperator => 1,
        _ => 0,
    }
}

fn infix_to_tree(infix: &[Token]) -> Node {
    // convert to postfix because fu
    let mut postfix: Vec<&Token> = Vec::new();
    let mut stack: Vec<&Token> = Vec::new();
    for op in infix {
        if op.tk_type == TokenType::Literal || op.tk_type == TokenType::Identifier {
            postfix.push(op);
        } else if op.tk_type == TokenType::LParen {
            stack.push(op);
        } else if op.tk_type == TokenType::RParen {
            while stack.len() > 0 && stack.last().unwrap().tk_type != TokenType::LParen {
                postfix.push(stack.pop().unwrap());
            }
            stack.pop();
        } else {
            while stack.len() > 0 && precedence(&stack.last().unwrap().lexeme) >= precedence(&op.lexeme) {
                postfix.push(stack.pop().unwrap());
            }
            stack.push(op);
        }
    }
    while stack.len() > 0 {
        postfix.push(stack.pop().unwrap());
    }

    let mut root = Node {
        construct: Construct::Operator, // root should be an operator
        value: None,
        children: Vec::new(),
    };
    let parent = &mut root;
    for t in &postfix {
        //println!("D: {}", t);
        if t.tk_type == TokenType::Literal || t.tk_type == TokenType::Identifier {
            stack.push(&t);
        } else {
            let op2 = Node {
                construct: Construct::Literal,
                value: Some(stack.pop().unwrap().lexeme.to_string()),
                children: Vec::new(),
            };
            let op1 = Node {
                construct: Construct::Literal,
                value: Some(stack.pop().unwrap().lexeme.to_string()),
                children: Vec::new(),
            };
            parent.value = Some(t.lexeme.to_string());
            parent.children.push(op1);
            parent.children.push(op2);
        }
    }
    return root;
}

pub fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    let mut root = Node {
        construct: Construct::Scope,
        value: None,
        children: Vec::new(),
    };
    let mut idx: usize = 0;
    let mut scope_stack: Vec<&mut Node> = Vec::new();
    scope_stack.push(&mut root);
    while idx < tokens.len() {
        match tokens[idx].tk_type {
            TokenType::Initialize => {
                idx += 1;
                if tokens[idx].tk_type == TokenType::Identifier {
                    let tk_var = &tokens[idx];
                    idx += 1;
                    if tokens[idx].tk_type == TokenType::Assign {
                        idx += 1;
                        let mut assign = Node {
                            construct: Construct::Assign,
                            value: None,
                            children: Vec::new(),
                        };
                        let var = Node {
                            construct: Construct::Variable,
                            value: Some(tk_var.lexeme.to_string()),
                            children: Vec::new(),
                        };
                        let end = expression_endpoint(tokens, idx).unwrap();
                        let expression_tokens = &tokens[idx..end];
                        let expression = infix_to_tree(expression_tokens);
                        assign.children.push(var);
                        assign.children.push(expression);
                        scope_stack.last_mut().unwrap().children.push(assign);
                    } else {
                        let error_message = std::format!("EXPECTED ASSIGN OPERATOR, FOUND '{}'", tokens[idx].lexeme);
                        return Err(error_message);
                    }
                } else {
                    let error_message = std::format!("EXPECTED IDENTIFIER SYMBOL, FOUND '{}'", tokens[idx].lexeme);
                    return Err(error_message);
                }
            },
            TokenType::Output => {
                idx += 1;
                let mut output = Node {
                    construct: Construct::Output,
                    value: None,
                    children: Vec::new(),
                };
                match tokens[idx].tk_type {
                    TokenType::Identifier => {
                        let expression = Node {
                            construct: Construct::Variable,
                            value: Some(tokens[idx].lexeme.to_string()),
                            children: Vec::new(),
                        };
                        output.children.push(expression);
                    },
                    _ => {},
                }
                scope_stack.last_mut().unwrap().children.push(output);
            },
            TokenType::If => {
                idx += 1;
                let mut branch = Node {
                    construct: Construct::Branch,
                    value: None,
                    children: Vec::new(),
                };
                let end = expression_endpoint(tokens, idx).unwrap();
                let condition_tokens = &tokens[idx..end];
                let tree = infix_to_tree(condition_tokens);
                branch.children.push(tree);
                scope_stack.last_mut().unwrap().children.push(branch);
            },
            //TokenType::EndScope => {
              //  if scope_stack.len() == 1 {
                //    return Err("EXTRA END SCOPE KEYWORD FOUND".to_string());
                //}
                //scope_stack.pop();
            //}
            _ => {},
        }
        idx += 1;
    }
    Ok(root)
}
