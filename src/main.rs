use colored::Colorize;

use rustyline::error::ReadlineError;
use rustyline::Editor;
// use rustyline::EditMode;
// use rustyline::Config;

mod calc;
mod ops;
mod stack;
mod parse;

fn print_hello() {
    println!("alternating current desk calculator v{}", env!("CARGO_PKG_VERSION").blue());
    println!("operating in {} mode, with {} keybinds.", "floating point".yellow(), "emacs".green());
}

fn main() {
    print_hello();
    let mut calc = calc::Calculator::new();

    // set vim mode by default
    // let rl_config = Config::builder()
    //     .edit_mode(EditMode::Vi)
    //     .build();
    let mut rl = Editor::<()>::new();

    // load in a history
    if rl.load_history("/tmp/acdc_history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                // println!("Line: {}", line);
                calc.input(&line);

                if calc.should_quit() {
                    break
                }
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
    rl.save_history("/tmp/acdc_history.txt").unwrap();
}

