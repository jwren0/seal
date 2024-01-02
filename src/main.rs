use std::io::{self, Write};

mod repl;
use repl::Evaluator;

fn write_prompt() -> Result<(), io::Error> {
    print!("> ");

    io::stdout().flush()
}

fn main() -> Result<(), io::Error> {
    let mut eval = Evaluator::new();

    loop {
        write_prompt()?;

        let mut line = String::new();

        // Handle EOF
        if io::stdin().read_line(&mut line)? < 1 {
            println!();
            break;
        }

        match line.trim() {
            "" => continue,
            "exit" | "quit" | "q" => break,
            content => eval.run(&content),
        }
    }

    Ok(())
}
