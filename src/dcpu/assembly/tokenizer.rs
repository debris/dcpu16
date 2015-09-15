trait Bits {
    fn to_bits(&self) -> u8;
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    SET,
    ADD,
    SUB,
    MUL,
    MLI,
    DIV,
    DVI,
    MOD,
    MDI,
    AND,
    BOR,
    XOR
}

impl Bits for Opcode {
    fn to_bits(&self) -> u8 {
        match *self {
            Opcode::SET => 0x01,
            Opcode::ADD => 0x02,
            Opcode::SUB => 0x03,
            Opcode::MUL => 0x04,
            Opcode::MLI => 0x05,
            Opcode::DIV => 0x06,
            Opcode::DVI => 0x07,
            Opcode::MOD => 0x08,
            Opcode::MDI => 0x09,
            Opcode::AND => 0x0a,
            Opcode::BOR => 0x0b,
            Opcode::XOR => 0x0c
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Constant {
    A, B, C, X, Y, Z, I, J
}

impl Bits for Constant {
    fn to_bits(&self) -> u8 {
        match *self {
            Constant::A => 0x0,
            Constant::B => 0x1,
            Constant::C => 0x2,
            Constant::X => 0x3,
            Constant::Y => 0x4,
            Constant::Z => 0x5,
            Constant::I => 0x6,
            Constant::J => 0x7,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Value<'a> {
    Number(u16),
    Name(&'a str),
    Constant(Constant)
}

impl<'a> Bits for Value<'a> {
    fn to_bits(&self) -> u8 {
        match *self {
            Value::Number(n) => (n + 0x21) as u8,
            Value::Constant(c) => c.to_bits(),
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token<'a> {
    Opcode(Opcode),
    Value(Value<'a>),
    Whitespace,
    Comma,
    Endline,
    Invalid(&'a str, usize)
}

impl<'a> Bits for Token<'a> {
    fn to_bits(&self) -> u8 {
        match *self {
            Token::Opcode(op) => op.to_bits(),
            Token::Value(v) => v.to_bits(),
            _ => panic!()
        }
    }
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
            "B" | "b" => Token::Value(Value::Constant(Constant::B)),
            "C" | "c" => Token::Value(Value::Constant(Constant::C)),
            "X" | "x" => Token::Value(Value::Constant(Constant::X)),
            "Y" | "y" => Token::Value(Value::Constant(Constant::Y)),
            "Z" | "z" => Token::Value(Value::Constant(Constant::Z)),
            "I" | "i" => Token::Value(Value::Constant(Constant::I)),
            "J" | "j" => Token::Value(Value::Constant(Constant::J)),
            "SET" | "set" => Token::Opcode(Opcode::SET),
            "ADD" | "add" => Token::Opcode(Opcode::ADD),
            "SUB" | "sub" => Token::Opcode(Opcode::SUB),
            "MUL" | "mul" => Token::Opcode(Opcode::MUL),
            "MLI" | "mli" => Token::Opcode(Opcode::MLI),
            "DIV" | "div" => Token::Opcode(Opcode::DIV),
            "DVI" | "dvi" => Token::Opcode(Opcode::DVI),
            "MOD" | "mod" => Token::Opcode(Opcode::MOD),
            "MDI" | "mdi" => Token::Opcode(Opcode::MDI),
            "AND" | "and" => Token::Opcode(Opcode::AND),
            "BOR" | "bor" => Token::Opcode(Opcode::BOR),
            "XOR" | "xor" => Token::Opcode(Opcode::XOR),
            n @ _ => Token::Invalid(n, start_position)
        }
    }
}

struct LookaheadTokenizer<'a> {
    tokenizer: Tokenizer<'a>,
    cache: Vec<Option<Token<'a>>>
}

impl<'a> LookaheadTokenizer<'a> {
    pub fn new(source: &str) -> LookaheadTokenizer {
        LookaheadTokenizer {
            tokenizer: Tokenizer::new(source),
            cache: vec![]
        }
    }

    fn token_at(&mut self, position: usize) -> Option<Token<'a>> {
        self.load_until(position);
        self.cache[position]
    }

    fn load_until(&mut self, n: usize) {
        while self.cache.len() <= n {
            self.cache.push(self.tokenizer.next_token());
        }
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        for i in 0..n {
            self.cache.remove(0);
        }
    }
}

pub struct Parser<'a> {
    tokenizer: LookaheadTokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(source: &str) -> Parser {
        Parser {
            tokenizer: LookaheadTokenizer::new(source)
        }
    }

    fn skip_whitesigns(&mut self) {
        while matches!(self.tokenizer.token_at(0), Some(Token::Whitespace) | Some(Token::Endline)) {
            self.tokenizer.advance(1);
        }
    }

    fn is_opcode(&mut self, n: usize) -> bool {
        matches!(self.tokenizer.token_at(n), Some(Token::Opcode(ref op)))
    }

    fn is_whitespace(&mut self, n: usize) -> bool {
        matches!(self.tokenizer.token_at(n), Some(Token::Whitespace))
    }

    fn is_value(&mut self, n: usize) -> bool {
        matches!(self.tokenizer.token_at(n), Some(Token::Value(ref a)))
    }
    
    fn is_comma(&mut self, n: usize) -> bool {
        matches!(self.tokenizer.token_at(n), Some(Token::Comma))
    }

    fn parse_expression(&mut self) -> u16 {
        if (self.is_opcode(0) &&
            self.is_whitespace(1) &&
            self.is_value(2) &&
            self.is_comma(3) &&
            self.is_whitespace(4) &&
            self.is_value(5)) {
            let op = self.tokenizer.token_at(0).unwrap().to_bits() as u16;
            let b = self.tokenizer.token_at(2).unwrap().to_bits() as u16;
            let a = self.tokenizer.token_at(5).unwrap().to_bits() as u16;
            self.tokenizer.advance(6);
            return op + (b << 5) + (a << 10);
        }
        panic!()
    }

    pub fn parse(&mut self) -> Vec<u16> {
        let mut result = vec!();
        self.skip_whitesigns();
        while self.tokenizer.token_at(0).is_some() {
            result.push(self.parse_expression());
            self.skip_whitesigns();
        }
        result
    }
}

#[test]
fn test_parse_expression() {
    let mut parser = Parser::new("SET A, 30");
    assert_eq!(parser.parse_expression(), 0xfc01);
}

#[test]
fn test_parse() {
    let mut parser = Parser::new("SET A, 30");
    assert_eq!(parser.parse(), [0xfc01]);
}

#[test]
fn test_parse2() {
    let mut parser = Parser::new("\nSET   A,  30\n\n");
    assert_eq!(parser.parse(), [0xfc01]);
}

#[test]
fn test_parse3() {
    let mut parser = Parser::new("SET A, 30\n
                                  SET A, 1");
    assert_eq!(parser.parse(), [
               0xfc01, // SET A, 30
               0x8801  // SET A, 1
    ]);
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

