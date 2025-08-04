struct Lexer<'a> {
    tokens: Vec<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            tokens: content.split(";").collect(),
        }
    }
    pub fn get(&self, index: usize) -> Option<&&'a str> {
        self.tokens.get(index)
    }
}

struct Parser<'a> {
    content: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        let lexer = Lexer::new(content);
        Parser { content, lexer }
    }
    pub fn get(&self, index: usize) -> &'a str {
        self.lexer.get(index).unwrap();
        self.lexer.get(index).unwrap()
    }
    pub fn parse(self) -> &'a str {
        self.get(0)
    }
}

pub fn test<'a>(content: &'a str) -> &'a str {
    let list_obj = Parser::new(content);
    list_obj.get(0);
    let a = list_obj.get(0);
    a
}
