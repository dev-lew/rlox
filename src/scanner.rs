use std::iter::{Peekable, Take};
use std::str::Chars;

pub(crate) enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub(crate) enum ScanResult<'a> {
    Normal(Token<Take<Peekable<Chars<'a>>>>),
    EOF(EOFToken),
    Error(ErrorToken<'a>),
}

pub(crate) struct Token<I>
where
    I: Iterator,
{
    pub(crate) r#type: TokenType,
    lexeme: I,
    pub(crate) length: usize,
    pub(crate) line: i32,
}

// impl<'a> Token<Chars<'a>> {
//     // Investigate whether using slice_from_raw_parts(lexeme.as_str().as_ptr(), length)
//     // improves memory usage and or speed, due to not having to allocate memory for String
//     pub(crate) fn get_lexeme(&mut self) -> String {
//         self.lexeme.clone().take(self.length).collect()
//     }
// }

impl<'a> Token<Take<Peekable<Chars<'a>>>> {
    pub(crate) fn get_lexeme(&mut self) -> String {
        self.lexeme.clone().collect()
    }
}

pub(crate) struct EOFToken {
    pub(crate) line: i32,
}

impl EOFToken {
    pub(crate) fn new(line: i32) -> Self {
        Self { line }
    }
}

pub(crate) struct ErrorToken<'a> {
    pub(crate) message: &'a str,
}

impl<'a> ErrorToken<'a> {
    pub(crate) fn new(message: &'a str) -> Self {
        Self { message }
    }
}

pub(crate) struct Scanner<'a> {
    pub(crate) start: Peekable<Chars<'a>>,
    pub(crate) current: Peekable<Chars<'a>>,
    pub(crate) current_index_in_lexeme: usize,
    pub(crate) line: i32,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            start: source.chars().peekable(),
            current: source.chars().peekable(),
            current_index_in_lexeme: 0,
            line: 1,
        }
    }

    pub(crate) fn make_token(&self, r#type: TokenType) -> Token<Take<Peekable<Chars<'a>>>> {
        Token {
            r#type,
            lexeme: self.get_lexeme(),
            length: self.current_index_in_lexeme,
            line: self.line,
        }
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        let mut binding = self.current.clone();
        let current = binding.peek();

        if let Some(c) = self.current.next() {
            if !c.is_whitespace() {
                self.current_index_in_lexeme += 1;
            }
        }

        match current {
            Some(c_ref) => Some(*c_ref),
            None => None,
        }
    }

    pub(crate) fn scan_token(&mut self) -> ScanResult<'a> {
        self.skip_whitespace();

        self.start = self.current.clone();
        // let mut binding = self.current.clone();

        // if let Some(test) = binding.peek() {
        //     println!("scan token test: {}", test);
	// }

        self.current_index_in_lexeme = 0usize;

        if self.is_at_end() {
            ScanResult::EOF(EOFToken::new(self.line))
        } else {
            let c = self.advance();
            // println!("Scan token...");

            match c {
                Some('(') => ScanResult::Normal(self.make_token(TokenType::LeftParen)),
                Some(')') => ScanResult::Normal(self.make_token(TokenType::RightParen)),
                Some('{') => ScanResult::Normal(self.make_token(TokenType::LeftBrace)),
                Some('}') => ScanResult::Normal(self.make_token(TokenType::RightBrace)),
                Some(';') => ScanResult::Normal(self.make_token(TokenType::Semicolon)),
                Some(',') => ScanResult::Normal(self.make_token(TokenType::Comma)),
                Some('.') => ScanResult::Normal(self.make_token(TokenType::Dot)),
                Some('-') => ScanResult::Normal(self.make_token(TokenType::Minus)),
                Some('+') => ScanResult::Normal(self.make_token(TokenType::Plus)),
                Some('/') => ScanResult::Normal(self.make_token(TokenType::Slash)),
                Some('*') => ScanResult::Normal(self.make_token(TokenType::Star)),
                Some('!') => {
                    let token = if self.matches('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    ScanResult::Normal(self.make_token(token))
                }
                Some('=') => {
                    let token = if self.matches('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    ScanResult::Normal(self.make_token(token))
                }
                Some('<') => {
                    let token = if self.matches('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    ScanResult::Normal(self.make_token(token))
                }
                Some('>') => {
                    let token = if self.matches('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    ScanResult::Normal(self.make_token(token))
                }
                Some('"') => self.string(),
                Some(ident_or_digit) => {
                    if self.is_alpha(ident_or_digit) {
                        self.identifier()
                    } else if ident_or_digit.is_digit(10) {
                        self.number()
                    } else {
                        ScanResult::Error(ErrorToken::new("Unexpected character"))
                    }
                }
                None => ScanResult::Error(ErrorToken::new("Unexpected character")),
            }
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        let mut current = self.current.clone();

        match current.peek() {
            Some(c_ref) => {
                if self.is_at_end() || *c_ref != expected {
                    false
                } else {
                    self.current.next();
                    self.current_index_in_lexeme += 1;
                    true
                }
            }
            _ => panic!("is_at_end() arm should have been true!"),
        }
    }

    fn get_lexeme(&self) -> Take<Peekable<Chars<'a>>> {
        let test: String = self
            .start
            .clone()
            .take(self.current_index_in_lexeme)
            .collect();
        // println!(
        //     "lex from scanner get lexeme: {} {}",
        //     test, self.current_index_in_lexeme
        // );
        self.start.clone().take(self.current_index_in_lexeme)
    }

    fn is_at_end(&mut self) -> bool {
        self.current.peek().is_none()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn peek_next(&mut self) -> Option<char> {
        self.current.by_ref().nth(1)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current.peek() {
                Some('\n') => {
		    // println!("newline detected");
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if let Some('/') = self.peek_next() {
                        while let Some(c_ref) = self.current.peek() {
                            if *c_ref == '\n' {
                                break;
                            } else {
                                self.advance();
                            }
                        }
                    }
                }
                Some(c) => {
                    if c.is_whitespace() {
                        self.advance();
                    } else {
                        // println!("In whitespace");
                        return;
                    }
                }
                None => return,
            }
        }
    }

    fn string(&mut self) -> ScanResult<'a> {
        if let Some(c_ref) = self.current.peek() {
	    let mut binding = *c_ref;

	    while binding != '"' {
		if binding == '\n' {
		    self.line += 1;
		}

		match self.advance() {
		    Some(next) => {binding = next},
		    None => break,
		}
	    }
        }

        if self.is_at_end() {
            ScanResult::Error(ErrorToken::new("Unterminated string"))
        } else {
            self.advance();
            ScanResult::Normal(self.make_token(TokenType::String))
        }
    }

    fn number(&mut self) -> ScanResult<'a> {
        if let Some(c_ref) = self.current.peek() {
            let mut binding = *c_ref;

            while binding.is_digit(10) {
                match self.advance() {
                    Some(next) => binding = next,
                    None => break,
                }
            }
        }

        if let Some('.') = self.current.peek() {
            if let Some(c) = self.peek_next() {
                if c.is_digit(10) {
                    self.advance();

                    if let Some(c_ref) = self.current.peek() {
                        let mut binding = *c_ref;

                        while binding.is_digit(10) {
                            match self.advance() {
                                Some(next) => binding = next,
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        ScanResult::Normal(self.make_token(TokenType::Number))
    }

    fn identifier(&mut self) -> ScanResult<'a> {
        // println!("In identifier");
        let mut current = self.current.clone();

        if let Some(c_ref) = current.peek() {
            let mut binding = *c_ref;

            while self.is_alpha(binding) || binding.is_digit(10) {
                match self.advance() {
                    Some(next) => {
                        // println!("char: {}", next);
                        binding = next
                    }
                    None => break,
                }
            }

            // println!("char now: {}", self.current_index_in_lexeme);
        }
        let ident_type = self.identifier_type();
        ScanResult::Normal(self.make_token(ident_type))
    }

    fn identifier_type(&mut self) -> TokenType {
        // Figure out if we really need to clone here
        let mut ident_or_keyword = self.start.clone();

        match ident_or_keyword.next() {
            Some('a') => self.check_keyword("nd", &mut ident_or_keyword, TokenType::And),
            Some('c') => self.check_keyword("lass", &mut ident_or_keyword, TokenType::Class),
            Some('e') => self.check_keyword("lse", &mut ident_or_keyword, TokenType::Else),
            Some('i') => self.check_keyword("f", &mut ident_or_keyword, TokenType::If),
            Some('n') => self.check_keyword("nil", &mut ident_or_keyword, TokenType::Nil),
            Some('o') => self.check_keyword("r", &mut ident_or_keyword, TokenType::Or),
            Some('p') => self.check_keyword("rint", &mut ident_or_keyword, TokenType::Print),
            Some('r') => self.check_keyword("eturn", &mut ident_or_keyword, TokenType::Return),
            Some('s') => self.check_keyword("uper", &mut ident_or_keyword, TokenType::Super),
            Some('v') => self.check_keyword("ar", &mut ident_or_keyword, TokenType::Var),
            Some('w') => self.check_keyword("hile", &mut ident_or_keyword, TokenType::While),
            Some('f') => {
                if let Some(c) = ident_or_keyword.next() {
                    match c {
                        'a' => self.check_keyword("lse", &mut ident_or_keyword, TokenType::False),
                        'o' => self.check_keyword("r", &mut ident_or_keyword, TokenType::For),
                        'u' => self.check_keyword("n", &mut ident_or_keyword, TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            Some('t') => {
                if let Some(c) = ident_or_keyword.next() {
                    match c {
                        'h' => self.check_keyword("is", &mut ident_or_keyword, TokenType::This),
                        'r' => self.check_keyword("rue", &mut ident_or_keyword, TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(
        &mut self,
        rest: &str,
        lexeme: &mut Peekable<Chars<'a>>,
        r#type: TokenType,
    ) -> TokenType {
        let len = rest.len();

        if lexeme.clone().take(len).eq(rest.chars()) {
            r#type
        } else {
            TokenType::Identifier
        }
    }
}
