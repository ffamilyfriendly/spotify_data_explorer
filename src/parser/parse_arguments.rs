pub struct State {
    cursor: usize,
    buf: Vec<char>
}

impl State {
    pub fn new(from: &str) -> Self {
        State {
            cursor: 0,
            buf: from.chars().collect()
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.buf.get(self.cursor)
    }

    pub fn pop(&mut self) -> Option<&char> {
        match self.buf.get(self.cursor) {
            Some(val) => {
                self.cursor += 1;
                Some(&val)
            },
            None => None
        }
    }
}

pub enum Error {
    UnexpendedEnd(usize)
}

pub fn is_string(s: &str) -> Result<String, Error> {
    let mut state = State::new(s);
    state.pop();

    let mut b: Vec<&char> = Vec::new();

    loop {
        match state.peek() {
            Some('"') => {
                if state.cursor != 0 {
                    state.pop();
                    break;
                }

                state.pop();
            },
            Some(val) => {
                b.push(val);
                state.pop();
                return Ok("hello".to_owned())
            },
            None => return Err(Error::UnexpendedEnd(state.cursor()))
        };
    }
    

    Ok("HELLO".to_owned())

}

