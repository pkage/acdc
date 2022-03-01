use colored::Colorize;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod calc;
mod ops;
mod stack;
mod parse;

fn print_hello() {
    println!("alternating current desk calculator v{}", env!("CARGO_PKG_VERSION").blue());
    println!("operating in {} mode, with {} keybinds.", "floating point".yellow(), "vim".green());
}

fn main() {
    print_hello();
    // `()` can be used when no completer is required
    let mut calc = calc::Calculator::new();
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                // println!("Line: {}", line);
                calc.input(&line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

