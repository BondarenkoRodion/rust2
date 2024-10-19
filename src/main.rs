use std::collections::VecDeque;
use std::io;

fn main() {
    let mut last_result: Option<f64> = None;

    loop {
        println!("Введіть вираз ('m' для останнього видатку):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не вдалось прочитати введення.");
        let input = input.trim();

        let expression = if input.contains('m') {
            match last_result {
                Some(result) => input.replace("m", &result.to_string()),
                None => {
                    println!("'m' ще не має значення.");
                    continue;
                }
            }
        } else {
            input.to_string()
        };

        match evaluate_expression(&expression) {
            Ok(result) => {
                println!("Здобуток: {}", result);
                last_result = Some(result);
            }
            Err(e) => println!("Помилка: {}", e),
        }
    }
}

fn get_precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
    let mut output_queue: VecDeque<f64> = VecDeque::new();
    let mut operator_stack: Vec<char> = Vec::new();

    let mut num_buffer = String::new();

    for c in expression.chars() {
        if c.is_digit(10) || c == '.' {
            num_buffer.push(c);
        } else if matches!(c, '+' | '-' | '*' | '/') {
            if !num_buffer.is_empty() {
                let num = num_buffer.parse::<f64>().map_err(|_| "Невірне число.")?;
                output_queue.push_back(num);
                num_buffer.clear();
            }
            while let Some(&op) = operator_stack.last() {
                if get_precedence(op) >= get_precedence(c) {
                    operator_stack.pop();
                    apply_operator(&mut output_queue, op)?;
                } else {
                    break;
                }
            }
            operator_stack.push(c);
        }
        else if !matches!(c, ' ') {
            return Err("Неочікуваний знак");
        }
    }

    if !num_buffer.is_empty() {
        let num = num_buffer.parse::<f64>().map_err(|_| "Невірне число.")?;
        output_queue.push_back(num);
    }

    while let Some(op) = operator_stack.pop() {
        apply_operator(&mut output_queue, op)?;
    }

    if output_queue.len() == 1 {
        Ok(output_queue.pop_front().unwrap())
    } else {
        Err("Помилка в обчисленнях.")
    }
}

fn apply_operator(queue: &mut VecDeque<f64>, operator: char) -> Result<(), &'static str> {
    if queue.len() < 2 {
        return Err("Невистачає операндів.");
    }

    let b = queue.pop_back().unwrap();
    let a = queue.pop_back().unwrap();
    let result = match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                return Err("Ділення на нуль неможливе.");
            }
            a / b
        }
        _ => return Err("Невідомий оператор."),
    };
    queue.push_back(result);
    Ok(())
}
