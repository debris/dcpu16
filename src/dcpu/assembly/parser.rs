use super::tokenizer::Token as Token;
use super::tokenizer::Bits as Bits;
use super::tokenizer::LookaheadTokenizer as LookaheadTokenizer;

//struct ParseError<'a> {
    //string: &'a str,
    //position: usize
//}

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

    fn bits_at(&mut self, n: usize) -> u16 {
        self.tokenizer.token_at(n).unwrap().to_bits() as u16
    }

    fn parse_expression(&mut self) -> u16 {
        if (self.is_opcode(0) &&
            self.is_whitespace(1) &&
            self.is_value(2) &&
            self.is_comma(3) &&
            self.is_whitespace(4) &&
            self.is_value(5)) {
            let op = self.bits_at(0);
            let b = self.bits_at(2);
            let a = self.bits_at(5);
            self.tokenizer.advance(6);
            return op + (b << 5) + (a << 10);
        }

        if (self.is_opcode(0) &&
            self.is_whitespace(1) &&
            self.is_value(2)) {
            let op = self.bits_at(0);
            let a = self.bits_at(2);
            self.tokenizer.advance(3);
            return (op << 5) + (a << 10);
        }
        panic!();
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
