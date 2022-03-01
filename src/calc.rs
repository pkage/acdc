use colored::Colorize;

use crate::stack::*;
use crate::parse::*;
use crate::ops::*;

pub struct Calculator {
    stack: Stack,
    ops: Vec<Operation>,
    parser: Parser,
    should_quit: bool
}

fn print_repeated_chars(s: &String, c: &String) {
    let length = s.len();
    for _ in 0..length {
        print!("{}", c)
    }
}

impl Calculator {
    pub fn new() -> Calculator {
        
        let all_ops = get_all_operations();

        let mut parser = Parser::new();
        parser.set_ops_list(
            all_ops
                .iter()
                .map(|op| op.name.clone())
                .collect()
        );

        parser.set_special_list(vec![
                "help",
                "?",
                "quit",
                "q"
            ]
        );

        let stack = Stack::new();

        let calc = Calculator {
            stack,
            ops: get_all_operations(),
            parser,
            should_quit: false
        };

        return calc
    }

    fn show_error(&self, parses: &Vec<Parsed<f64>>) {
        print!("{}\n    ", "Error in input!".red());
        for parse in parses {
            match parse {
                Parsed::Invalid(s) => print!("{}", s.red()),
                Parsed::Number(n)  => print!("{}", n),
                Parsed::Operation(s) => print!("{}", s),
                Parsed::Special(s) => print!("{}", s)
            }

            print!(" ")
        }
        print!("\n    ");

        for parse in parses {
            match parse {
                Parsed::Invalid(s) => print_repeated_chars(&s, &"^".red().to_string()),
                Parsed::Number(n)  => print_repeated_chars(&format!("{}", n), &" ".to_string()),
                Parsed::Operation(s) => print_repeated_chars(&s, &" ".to_string()),
                Parsed::Special(s) => print_repeated_chars(&s, &" ".to_string()),
            }

            print!(" ")
        }

        println!("");

        for parse in parses {
            match parse {
                Parsed::Invalid(s) => {
                    println!("unknown: {}", s.red())
                },
                _ => ()
            }
        }
        
        println!("{}", "Ignoring input.".yellow());
    }

    fn add_to_stack(&mut self, number: f64) {
        self.stack.push(number)
    }

    fn get_op_by_name(&self, name: String) -> Result<&Operation, ()> {
        for op in &self.ops {
            if op.name == name {
                return Ok(op)
            }
        }

        return Err(())
    }

    pub fn should_quit(&self) -> bool {
        return self.should_quit
    }

    fn show_help(&self) {
        println!("Available operations:");
        for op in &self.ops {
            println!("    {:4} | {}", op.name.green(), op.desc);
        }

        println!("Other operations: ");
        println!("    {}", "help ? quit q".yellow());

    }

    fn dispatch_special(&mut self, special: &str) {
        match special {
            "help" | "?" => self.show_help(),
            "quit" | "q" => self.should_quit = true,
            _ => println!("uh oh")
        }
    }

    pub fn input(&mut self, line: &String) {
        let parses = self.parser.parse_line::<f64>(line);

        /*
        println!("Parse table: ");
        
        let mut lineno = 0;
        for parse in &parses {
            let s = match parse {
                Parsed::Number(n) => format!("number:{}", n).blue(),
                Parsed::Special(s) => format!("special:{}", s).yellow(),
                Parsed::Operation(s) => format!("op:{}", s).green(),
                Parsed::Invalid(s) => format!("invalid:{}", s).red(),
            };

            println!("{} : {}", lineno, s);
            lineno += 1;
        } 
        */

        for parse in &parses {
            match parse {
                Parsed::Invalid(_) => { self.show_error(&parses); return }
                _ => ()
            }
        }

        // as long as nothing is invalid, we can now dispatch
        for parse in &parses {
            match parse {
                Parsed::Number(n) => self.add_to_stack(*n),
                Parsed::Operation(s) => {
                    let op_maybe = self.get_op_by_name(s.clone());
                    match op_maybe {
                        Ok( op ) => { 
                            if op.arity > self.stack.size() {
                                println!("{}", format!("Stack is too shallow! Need at least {} items (have {}).", op.arity, self.stack.size()).red());
                                return
                            }
                            (op.func)(&mut self.stack);
                        },
                        Err(()) => { println!("unable to resolve op '{}'!", s); return }
                    }
                },
                Parsed::Special(s) => self.dispatch_special(&s),
                _=> ()
            }
        }

    }
}


