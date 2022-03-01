pub struct Stack {
    pub contents: Vec<f64>
}

impl Stack {
    pub fn new() -> Stack {
        return Stack { contents: vec![] }
    }
    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn push(&mut self, value: f64) -> () {
        self.contents.push(value)
    }

    pub fn peek(&self) -> Result<f64, ()> {
        return match self.size() {
            0 => Err(()),
            _ => Ok(self.contents[self.size() - 1])
        }
    }

    pub fn pop(&mut self) -> Result<f64, ()> {
        return match self.contents.pop() {
            Some(a) => Ok(a),
            None    => Err(())
        }
    }
}
