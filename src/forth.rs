use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type Name = String;
type Words = Vec<String>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<Name, Words>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let lexemes = input.to_ascii_lowercase();
        let mut lexemes = lexemes.split_whitespace().peekable();

        while let Some(token) = lexemes.next() {
            match token {
                "+" => self.add()?,
                "*" => self.multiply()?,
                "-" => self.subtract()?,
                "/" => self.divide()?,
                "dup" => self.duplicate()?,
                "drop" => self.drop()?,
                "swap" => self.swap()?,
                "over" => self.duplicate_second_top()?,
                ":" => {
                    let name = lexemes.next().ok_or(Error::InvalidWord)?;

                    let mut tks = Vec::new();
                    while let Some(tkn) = lexemes.next_if(|t| !t.eq(&";")) {
                        tks.push(tkn.to_string());
                    }

                    let sem_col = lexemes.next().ok_or(Error::InvalidWord)?;
                    if !sem_col.eq(";") {
                        return Err(Error::InvalidWord);
                    }

                    self.words.insert(name.to_string(), tks);
                }
                other => {
                    if let Some(tks) = self.words.get(other) {
                        self.eval(&tks.join(" "))?;
                    } else {
                        self.read_number(other)?
                    }
                }
            }
        }

        Ok(())
    }

    fn add(&mut self) -> Result {
        let fst = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let snd = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(fst + snd);
        Ok(())
    }

    fn multiply(&mut self) -> Result {
        let fst = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let snd = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(fst * snd);
        Ok(())
    }

    fn subtract(&mut self) -> Result {
        let rhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(lhs - rhs);
        Ok(())
    }

    fn divide(&mut self) -> Result {
        let rhs = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(Error::StackUnderflow)?;

        // Integer division drops the remainder
        let div = lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?;

        self.stack.push(div);

        Ok(())
    }

    fn duplicate(&mut self) -> Result {
        let val = self.stack.last().ok_or(Error::StackUnderflow)?;
        self.stack.push(*val);
        Ok(())
    }

    fn duplicate_second_top(&mut self) -> Result {
        // `nth_back` is zero-indexed
        let val = self.stack.iter().nth_back(1).ok_or(Error::StackUnderflow)?;
        self.stack.push(*val);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let fst = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let snd = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(fst);
        self.stack.push(snd);
        Ok(())
    }

    fn read_number(&mut self, input: &str) -> Result {
        let val = input.parse::<i32>().map_err(|_| Error::UnknownWord)?;
        self.stack.push(val);
        Ok(())
    }
}
