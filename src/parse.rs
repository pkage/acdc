pub enum Parsed<T> {
    Number(T),
    Special(String),
    Operation(String),
    Invalid(String)
}

pub const SPECIAL_TRIGGER_CHAR : char = ';';

pub struct Parser {
    op_list: Vec<String>
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            op_list: vec![]
        }
    }

    pub fn set_ops_list(&mut self, op_list: Vec<String>) -> () {
        self.op_list = op_list
    }

    /**
     * Parse a single line into a list of parses
     */
    pub fn parse_line<T: std::str::FromStr>(&self, line: &String) -> Vec<Parsed<T>> {
        let line_cleaned = line.trim_start().trim_end();
        if line_cleaned.len() > 1 {
            match line_cleaned.chars().nth(0) {
                Some(c) => {
                    if c == SPECIAL_TRIGGER_CHAR {
                        return vec![Parsed::<T>::Special(line_cleaned.to_string())];
                    }
                },
                None => ()
            }
        }

        let split = line.split_whitespace();

        let mut parses: Vec<Parsed<T>> = vec![];

        for s_raw in split {
            let s = s_raw.to_string();
            // println!("s: {}", s);

            if self.op_list.contains(&s) {
                parses.push(Parsed::Operation(s));
                continue
            }

            parses.push(match s.parse::<T>() {
                Ok(n) => Parsed::Number(n),
                Err(_) => Parsed::Invalid(s)
            })
        }

        return parses
    }

}


