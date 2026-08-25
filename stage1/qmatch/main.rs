use std::fs;
use std::io;

use rand::RngExt;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "qmatch")]
struct Args {
    #[arg(short, long)]
    level: u32,
}


fn print_banner() {
    let ascii_image = fs::read_to_string("./ascii/main.txt")
        .expect("Error fatal in print the banner");

    println!("{}", ascii_image);
}


fn generate_basic_expression() -> (String, String) {
    let mut rng = rand::rng();
    let expression_model: u32 = rng.random_range(1..=3);

    match expression_model {
        1 => {
            let operand1: i32 = rng.random_range(0..100);
            let operand2: i32 = rng.random_range(0..100);
            let operand3: i32 = rng.random_range(0..100);

            let answer = format!("{}", operand1 + operand2 + operand3);
            let expression = format!("{} + {} + {}", operand1, operand2, operand3);

            (answer, expression)
        }

        2 => {
            let operand1: i32 = rng.random_range(0..100);
            let operand2: i32 = rng.random_range(0..100);

            let answer = format!("{}", operand1.isqrt() + operand2.isqrt());
            let expression = format!("√{} + √{}", operand1, operand2);

            (answer, expression)
        }

        3 => {
            let numerator1: i32 = rng.random_range(1..100);
            let denominator1: i32 = rng.random_range(1..100);

            let numerator2: i32 = rng.random_range(1..100);
            let denominator2: i32 = rng.random_range(1..100);

            let result_numerator =
                numerator1 * denominator2 + numerator2 * denominator1;

            let result_denominator =
                denominator1 * denominator2;

            let answer = format!("{}/{}", result_numerator, result_denominator);

            let expression = format!(
                "{}/{} + {}/{}",
                numerator1,
                denominator1,
                numerator2,
                denominator2
            );

            (answer, expression)
        }

        _ => unreachable!()
    }
}


fn generate_numeric_expression() -> (String, String) {
    let mut rng = rand::rng();

    let expression_model: u32 = rng.random_range(1..=3);

    match expression_model {
        1 => {
            let a: i32 = rng.random_range(10..50);
            let b: i32 = rng.random_range(1..a);
            let c: i32 = rng.random_range(2..6);
            let exponent: u32 = rng.random_range(2..=3);

            let answer = (a - b) / c.pow(exponent);

            let expression = format!(
                "({} - {}) ÷ {}{}",
                a,
                b,
                c,
                powr(exponent as i32)
            );

            (answer.to_string(), expression)
        }

        2 => {
            let a: i32 = rng.random_range(1..20);
            let b: i32 = rng.random_range(1..20);
            let c: i32 = rng.random_range(1..20);

            let answer = a * (b + c);

            let expression = format!(
                "{} × ({} + {})",
                a,
                b,
                c
            );

            (answer.to_string(), expression)
        }

        3 => {
            let a: i32 = rng.random_range(1..100);
            let b: i32 = rng.random_range(1..50);

            let answer = a.isqrt() + b;

            let expression = format!(
                "√{} + {}",
                a,
                b
            );

            (answer.to_string(), expression)
        }

        _ => unreachable!()
    }
}


fn powr(n: i32) -> char {
    match n {
        0 => '⁰',
        1 => '¹',
        2 => '²',
        3 => '³',
        4 => '⁴',
        5 => '⁵',
        6 => '⁶',
        7 => '⁷',
        8 => '⁸',
        9 => '⁹',
        _ => '?',
    }
}


fn generate_question(level: u32) {
    if level == 1 {
        let (answer, expression) = generate_basic_expression();

        println!("{}", expression);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();

        if input == answer {
            println!("good job!, next question");
        } else {
            println!("error: expected {}", answer);
        }
    }


    if level == 2 {
        let (answer, expression) = generate_numeric_expression();

        println!("{}", expression);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();

        if input == answer {
            println!("good job!, next question");
        } else {
            println!("error: expected {}", answer);
        }
    }


    if level == 3 {
        println!("ps: format your response using n x2 y6");

        let mut rng = rand::rng();

        let n: i32 = rng.random_range(1..=20);
        let y_unknown: i32 = rng.random_range(0..=9);
        let x_unknown: i32 = rng.random_range(0..=9);

        let object_exponent: u32 = rng.random_range(0..=9);

        let y_result: i32 = y_unknown * object_exponent as i32;
        let x_result: i32 = x_unknown * object_exponent as i32;
        let n_result: i32 = n.pow(object_exponent);


        let response = format!(
            "{} x{} y{}",
            n_result,
            x_result,
            y_result,
        );


        println!(
            "({} x{} y{}){}",
            n,
            powr(x_unknown),
            powr(y_unknown),
            powr(object_exponent as i32),
        );


        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();


        if response == input {
            println!("good job! next question");
        } else {
            println!("error: expected {}", response);
        }
    }
}


fn game_loop(level: u32) {
    loop {
        generate_question(level);
    }
}


fn main() {
    let args = Args::parse();

    print_banner();
    game_loop(args.level);
}
