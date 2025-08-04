#[derive(Clone)]
pub struct Token<'a> {
    value: &'a str,
}

fn get_tokens(body: &str) -> Vec<Token<'_>> {
    vec![Token { value: &body[..2] }]
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    body: &'a str,
    cursor: usize,
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Vec<Token<'a>> {
        self.next();
        self.previous();
        vec![self.tokens[0].clone()]
    }

    fn next(&mut self) {
        self.cursor += 1;
    }
    fn previous(&mut self) {
        self.cursor -= 1;
    }
}

fn get_value<'a>(body: &'a str) -> Vec<Token<'a>> {
    let tokens = get_tokens(body);
    let mut parser = Parser {
        tokens,
        body,
        cursor: 0,
    };
    parser.parse()
}
