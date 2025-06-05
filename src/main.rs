fn main() {
    println!("Hello, world!");
    let code = ["Di estas levas 19", "Das 2"];
    for line in code {
        let tokens = line
            .split_whitespace()
            .map(|x| Token::parse(x))
            .collect::<Option<Vec<_>>>()
            .unwrap();
        dbg!(&tokens);
        dbg!(Expr::parse(tokens));
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Expr {
    Defun(String, Vec<Expr>),
    Let(String, Vec<Expr>),
    Call(String, Vec<Expr>),
    Add(Vec<Expr>),
    Number(f64),
    Variable(String),
}

impl Expr {
    fn parse(tokens: Vec<Token>) -> Option<Vec<Self>> {
        let tokens: Vec<_> = tokens.split(|x| matches!(x, Token::And)).collect();
        let exprgen = |tokens: Vec<Token>, n: usize| match tokens.get(n)? {
            Token::Infinitive(name) | Token::Verb(name) => match name.as_str() {
                "lev" => Some(Expr::Add(Expr::parse(tokens.get(n + 1..)?.to_vec())?)),
                "est" => match tokens.first()? {
                    Token::Noun(name) => Some(Expr::Let(
                        name.to_owned(),
                        Expr::parse(tokens.get(n + 1..)?.to_vec())?,
                    )),
                    Token::Infinitive(name) => Some(Expr::Defun(
                        name.to_owned(),
                        Expr::parse(tokens.get(n + 1..)?.to_vec())?,
                    )),
                    _ => None,
                },
                name => Some(Expr::Call(
                    name.to_string(),
                    Expr::parse(tokens.get(n + 1..)?.to_vec())?,
                )),
            },
            Token::Number(n) => Some(Expr::Number(*n)),
            Token::Accusative(n) => match *n.clone() {
                Token::Noun(n) => Some(Expr::Variable(n)),
                _ => None,
            },
            _ => None,
        };
        tokens
            .iter()
            .map(|x| {
                let x = x.to_vec();
                if let Some(res) = exprgen(x.clone(), 0) {
                    Some(res)
                } else {
                    exprgen(x, 1)
                }
            })
            .collect()
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Token {
    Noun(String),
    Accusative(Box<Token>),
    Plural(Box<Token>),
    Adjective(String),
    Adverb(String),
    Verb(String),
    Infinitive(String),
    Number(f64),
    And,
}

impl Token {
    fn parse(source: &str) -> Option<Token> {
        let source = source.trim();
        if source == "kaj" {
            Some(Token::And)
        } else if let Some(source) = source.strip_suffix("o") {
            Some(Token::Noun(source.to_string()))
        } else if let Some(source) = source.strip_suffix("a") {
            Some(Token::Adjective(source.to_string()))
        } else if let Some(source) = source.strip_suffix("e") {
            Some(Token::Adverb(source.to_string()))
        } else if let Some(source) = source.strip_suffix("as") {
            Some(Token::Verb(source.to_string()))
        } else if let Some(source) = source.strip_suffix("i") {
            Some(Token::Infinitive(source.to_string()))
        } else if let Some(source) = source.strip_suffix("j") {
            Some(Token::Plural(Box::new(Token::parse(source)?)))
        } else if let Some(source) = source.strip_suffix("n") {
            Some(Token::Accusative(Box::new(Token::parse(source)?)))
        } else if let Ok(source) = source.parse::<f64>() {
            Some(Token::Number(source))
        } else {
            None
        }
    }
}
