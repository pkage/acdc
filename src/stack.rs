pub struct Stack<T> {
    pub contents: Vec<T>
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        return Stack { contents: vec![] }
    }
    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn push(&mut self, value: T) -> () {
        self.contents.push(value)
    }

    pub fn peek(&self) -> Result<T, ()> {
        return match self.size() {
            0 => Err(()),
            _ => Ok(self.contents[self.size() - 1])
        }
    }

    pub fn pop(&mut self) -> Result<T, ()> {
        return match self.contents.pop() {
            Some(a) => Ok(a),
            None    => Err(())
        }
    }
}
