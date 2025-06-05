fn main() {
    let code = "inci estas levas 1 kaj tion. no estas inci 2. levas 2 kaj non";
    println!("{}", run(code).unwrap());
}

fn run(code: &str) -> Option<String> {
    println!("Hello, world!");
    let mut result = String::new();
    for line in code.trim().split(".") {
        let tokens = line
            .split_whitespace()
            .map(|x| Token::parse(x))
            .collect::<Option<Vec<_>>>()?;
        let ast = Expr::parse(tokens)?;
        result.push_str(&ast.first()?.compile()?);
        result.push_str(";\n");
    }
    Some(result)
}

#[derive(PartialEq, Clone, Debug)]
enum Expr {
    Defun(String, Vec<Expr>),
    Let(String, Vec<Expr>),
    Call(String, Vec<Expr>),
    Add(Vec<Expr>),
    Number(isize),
    Variable(String),
}

impl Expr {
    fn parse(tokens: Vec<Token>) -> Option<Vec<Self>> {
        let tokenss: Vec<_> = tokens.split(|x| matches!(x, Token::And)).collect();
        let stmtgen = |tokens: Vec<Token>| match tokens.get(1)? {
            Token::Verb(name) => match name.as_str() {
                "est" => match tokens.get(0)? {
                    Token::Noun(name) => Some(Expr::Let(
                        name.to_owned(),
                        Expr::parse(tokens.get(2..)?.to_vec())?,
                    )),
                    Token::Infinitive(name) => Some(Expr::Defun(
                        name.to_owned(),
                        Expr::parse(tokens.get(2..)?.to_vec())?,
                    )),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        };
        let exprgen = |tokens: Vec<Token>| match tokens.get(0)? {
            Token::Infinitive(name) | Token::Verb(name) => match name.as_str() {
                "lev" => Some(Expr::Add(Expr::parse(tokens.get(1..)?.to_vec())?)),
                name => Some(Expr::Call(
                    name.to_string(),
                    Expr::parse(tokens.get(1..)?.to_vec())?,
                )),
            },
            Token::Number(n) => Some(Expr::Number(*n)),
            Token::Accusative(n) => match *n.clone() {
                Token::Noun(n) => Some(Expr::Variable(n)),
                _ => None,
            },
            _ => None,
        };
        let tried = |x: &&[Token]| {
            let x = x.to_vec();
            stmtgen(x.clone()).or_else(|| exprgen(x))
        };
        if tokenss.iter().all(|x| x.len() == 1) {
            tokenss.iter().map(tried).collect()
        } else {
            Some(vec![tried(&&tokens.as_slice())?])
        }
    }

    fn compile(&self) -> Option<String> {
        match self {
            Expr::Add(nums) => Some(
                nums.iter()
                    .map(|x| x.compile())
                    .collect::<Option<Vec<_>>>()?
                    .join(" + "),
            ),
            Expr::Defun(name, body) => Some(format!(
                "function {name}(ti) {{ return {} }}",
                body.first()?.compile()?
            )),
            Expr::Let(name, body) => Some(format!("let {name} = {}", body.first()?.compile()?)),
            Expr::Call(name, args) => Some(format!(
                "{name}({})",
                args.iter()
                    .map(|x| x.compile())
                    .collect::<Option<Vec<_>>>()?
                    .join(", ")
            )),
            Expr::Variable(v) => Some(v.to_owned()),
            Expr::Number(n) => Some(n.to_string()),
        }
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
    Number(isize),
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
        } else if let Ok(source) = source.parse::<isize>() {
            Some(Token::Number(source))
        } else {
            None
        }
    }
}
