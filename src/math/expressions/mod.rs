use crate::global::{get_var, set_var};

#[derive(Debug, Clone, PartialEq)]
enum MathToken {
    Number(f64),
    Variable(String),
    Operator(char),
    LeftParen,
    RightParen,
}

fn parse_math_tokens(expr: &str) -> Result<Vec<MathToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(MathToken::Number(num_str.parse::<f64>().map_err(|_| format!("Invalid number: {}", num_str))?));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut var_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        var_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(MathToken::Variable(var_str));
            }
            '+' | '-' | '/' | '%' => {
                tokens.push(MathToken::Operator(c));
                chars.next();
            }
            '*' => {
                chars.next();
                if chars.peek() == Some(&'*') {
                    chars.next();
                    tokens.push(MathToken::Operator('^')); // Use ^ internally for power
                } else {
                    tokens.push(MathToken::Operator('*'));
                }
            }
            '(' => {
                tokens.push(MathToken::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(MathToken::RightParen);
                chars.next();
            }
            ' ' => {
                chars.next(); // Skip whitespace
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }
    Ok(tokens)
}

fn get_precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn shunting_yard(tokens: Vec<MathToken>) -> Result<Vec<MathToken>, String> {
    let mut output_queue: Vec<MathToken> = Vec::new();
    let mut operator_stack: Vec<MathToken> = Vec::new();

    for token in tokens {
        match token {
            MathToken::Number(_) | MathToken::Variable(_) => output_queue.push(token),
            MathToken::Operator(op1) => {
                while let Some(MathToken::Operator(op2)) = operator_stack.last() {
                    if get_precedence(op1) <= get_precedence(*op2) {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(MathToken::Operator(op1));
            }
            MathToken::LeftParen => operator_stack.push(MathToken::LeftParen),
            MathToken::RightParen => {
                while let Some(top) = operator_stack.last() {
                    if *top == MathToken::LeftParen {
                        break;
                    }
                    output_queue.push(operator_stack.pop().unwrap());
                }
                if operator_stack.pop() != Some(MathToken::LeftParen) {
                    return Err("Mismatched parentheses".to_string());
                }
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op == MathToken::LeftParen {
            return Err("Mismatched parentheses".to_string());
        }
        output_queue.push(op);
    }

    Ok(output_queue)
}

fn evaluate_rpn(rpn_queue: Vec<MathToken>) -> Result<f64, String> {
    let mut value_stack: Vec<f64> = Vec::new();

    for token in rpn_queue {
        match token {
            MathToken::Number(n) => value_stack.push(n),
            MathToken::Variable(var_name) => {
                let val_str = get_var(&var_name);
                let val = val_str.parse::<f64>().map_err(|_| format!("Variable '{}' is not a valid number: {}", var_name, val_str))?;
                value_stack.push(val);
            }
            MathToken::Operator(op) => {
                let b = value_stack.pop().ok_or("Invalid expression: missing operand")?;
                let a = value_stack.pop().ok_or("Invalid expression: missing operand")?;
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    '%' => a % b,
                    '^' => a.powf(b),
                    _ => return Err(format!("Unknown operator: {}", op)),
                };
                value_stack.push(result);
            }
            _ => return Err("Invalid token in RPN queue".to_string()),
        }
    }

    if value_stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(value_stack[0])
}

/// Evaluates a mathematical expression.
/// Handles variable assignment ("VAR = ...") and shorthand assignments ("VAR += ...").
/// Supports operator precedence, parentheses, and floating-point numbers.
pub fn evaluate_expression(full_expr: &str) -> Result<f64, String> {
    let parts: Vec<&str> = full_expr.splitn(2, '=').map(|s| s.trim()).collect();

    let (target_var, expr_str) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err("Invalid assignment expression. Expected format: 'VAR = ...'".to_string());
    };

    let (op, var_name) = if let Some(stripped) = target_var.strip_suffix('+') {
        ('+', stripped.trim())
    } else if let Some(stripped) = target_var.strip_suffix('-') {
        ('-', stripped.trim())
    } else if let Some(stripped) = target_var.strip_suffix('*') {
        ('*', stripped.trim())
    } else if let Some(stripped) = target_var.strip_suffix('/') {
        ('/', stripped.trim())
    } else {
        ('=', target_var)
    };

    let tokens = parse_math_tokens(expr_str)?;
    let rpn = shunting_yard(tokens)?;
    let mut result = evaluate_rpn(rpn)?;

    if op != '=' {
        let current_val_str = get_var(var_name);
        let current_val = current_val_str.parse::<f64>().map_err(|_| format!("Variable '{}' for shorthand op is not a valid number: {}", var_name, current_val_str))?;
        result = match op {
            '+' => current_val + result,
            '-' => current_val - result,
            '*' => current_val * result,
            '/' => current_val / result,
            _ => result,
        };
    }

    set_var(var_name, &result.to_string());
    Ok(result)
}
