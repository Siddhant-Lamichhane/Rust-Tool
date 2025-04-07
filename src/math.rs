use log;

/// Does the Add subcommand.
pub fn handle_add(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("No numbers provided for addition");
        return;
    }
    let sum: f64 = numbers.iter().sum();
    println!("The Sum is: {}", sum);
    log::info!("Add subcommand executed with numbers: {:?}", numbers);
}

/// Does the Sub subcommand.
pub fn handle_sub(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("No numbers provided for subtraction");
        return;
    }
    let result = numbers[1..].iter().fold(numbers[0], |acc, &x| acc - x);
    println!("The Subtraction result is: {}", result);
    log::info!("Sub subcommand executed with numbers: {:?}", numbers);
}

/// Does the Mul subcommand.
pub fn handle_mul(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("No numbers provided for multiplication");
        return;
    }
    let product: f64 = numbers.iter().product();
    println!("The Product is: {}", product);
    log::info!("Mul subcommand executed with numbers: {:?}", numbers);
}

/// Does the Div subcommand.
pub fn handle_div(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("No numbers provided for division");
        return;
    }
    let mut iter = numbers.iter();
    let first = *iter.next().unwrap();
    let result = iter.try_fold(first, |acc, &x| {
        if x == 0.0 {
            Err("Division by zero encountered")
        } else {
            Ok(acc / x)
        }
    });
    match result {
        Ok(val) => {
            println!("The Division result is: {}", val);
            log::info!("Div subcommand executed with numbers: {:?}", numbers);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

