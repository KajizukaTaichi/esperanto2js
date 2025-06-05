fn main() {
    let code = "
        Hogi estas aldoni 1 kaj tion.
        Numero estas hogi aldoni 1 kaj 2.
        Mi multas 3 kaj numeron
    ";
    println!("{}", run(code).unwrap());
}

fn run(code: &str) -> Option<String> {
    let code = code.to_lowercase();
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
    Oper(String, Vec<Expr>),
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
                "aldon" => Some(Expr::Oper(
                    "+".to_owned(),
                    Expr::parse(tokens.get(1..)?.to_vec())?,
                )),
                "mult" => Some(Expr::Oper(
                    "*".to_owned(),
                    Expr::parse(tokens.get(1..)?.to_vec())?,
                )),
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
            let mut x = x.to_vec();
            match x.first()? {
                Token::Infinitive(name) if name == "m" => x = x.get(1..)?.to_vec(),
                _ => {}
            }
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
            Expr::Oper(op, nums) => Some(
                nums.iter()
                    .map(|x| x.compile())
                    .collect::<Option<Vec<_>>>()?
                    .join(op),
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
