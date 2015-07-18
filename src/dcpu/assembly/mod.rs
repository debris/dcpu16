#[derive(Debug, PartialEq)]
enum Opcode {
    SET,
    ADD
}

#[derive(Debug, PartialEq)]
enum Constant {
    A
}

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Number(u16),
    Name(&'a str),
    Constant(Constant)
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Opcode(Opcode),
    Value(Value<'a>),
    Whitespace,
    Comma,
    Endline,
    Invalid(&'a str, usize)
}

struct Tokenizer<'a> {
    source: &'a str,
    position: usize
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            source: source,
            position: 0
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        if self.is_eof() {
            return None
        }
        
        let c = self.next_char();
        match c {
            '\t' | ' ' | '\r' | '\x0C' => Some(self.consume_whitespace()),
            '\n' => {
                self.advance(1);
                Some(Token::Endline)
            },
            ',' => {
                self.advance(1);
                Some(Token::Comma)
            },
            '0' if self.has_at_least(1) && self.char_at(1) == 'x' => {
                self.advance(2); // consume 0x
                Some(self.consume_number(16))
            },
            '0'...'9' => Some(self.consume_number(10)),
            'a'...'z' | 'A'...'Z' => Some(self.consume_word()),
            '-' => None,
            _ => None
        }
    }
    
    #[inline]
    fn next_char(&self) -> char {
        self.char_at(0)
    }

    #[inline]
    fn char_at(&self, offset: usize) -> char {
        self.source[self.position + offset..].chars().next().unwrap()
    }
    
    #[inline]
    fn is_eof(&self) -> bool { 
        !self.has_at_least(0) 
    }
    
    #[inline]
    fn has_at_least(&self, n: usize) -> bool { 
        self.position + n < self.source.len() 
    }
    
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }
    
    #[inline]
    fn slice_from(&self, start_pos: usize) -> &'a str {
        &self.source[start_pos..self.position]
    }

    fn consume_whitespace(&mut self) -> Token<'a> {
        let mut start_position = self.position;
        while !self.is_eof() {
            match self.next_char() {
                '\t' | ' ' | '\r' | '\x0C' => self.advance(1),
                _ => break
            }
        }
        Token::Whitespace
    }

    fn consume_number(&mut self, radix: u32) -> Token<'a> {
        let mut start_position = self.position;
        while !self.is_eof() {
            match self.next_char() {
                '0'...'9' => self.advance(1),
                _ => break
            }
        }
        
        let slice = self.slice_from(start_position);
        match u16::from_str_radix(slice, radix) {
            Ok(v) => Token::Value(Value::Number(v)),
            Err(err) => Token::Invalid("parse error", start_position)
        }
    }

    fn consume_word(&mut self) -> Token<'a> {
        let mut start_position = self.position;
        while !self.is_eof() {
            match self.next_char() {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => self.advance(1),
                _ => break
            }
        }
        let slice = self.slice_from(start_position);
        match slice {
            "A" | "a" => Token::Value(Value::Constant(Constant::A)),
            "SET" | "set" => Token::Opcode(Opcode::SET),
            "ADD" | "add" => Token::Opcode(Opcode::ADD),
            n @ _ => Token::Invalid(n, start_position)
        }
    }
}

#[test]
fn test_token() {
    let mut tokenizer = Tokenizer::new("  ,,\n0x0 0x13 13 A");
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Comma));
    assert_eq!(tokenizer.next_token(), Some(Token::Comma));
    assert_eq!(tokenizer.next_token(), Some(Token::Endline));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Number(0))));
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Number(0x13))));
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Number(13))));
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Constant(Constant::A))));
    assert_eq!(tokenizer.next_token(), None);
}

#[test]
fn test_expression() {
    let mut tokenizer = Tokenizer::new("SET A, 15");
    assert_eq!(tokenizer.next_token(), Some(Token::Opcode(Opcode::SET)));
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Constant(Constant::A))));
    assert_eq!(tokenizer.next_token(), Some(Token::Comma));
    assert_eq!(tokenizer.next_token(), Some(Token::Whitespace));
    assert_eq!(tokenizer.next_token(), Some(Token::Value(Value::Number(15))));
}

