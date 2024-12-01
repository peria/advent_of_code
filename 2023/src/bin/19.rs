use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let buf = buf.trim();
    let p = buf.find("\n\n").unwrap();

    let system = System::from(&buf[..p]);

    let mut ratings = Vec::new();
    for line in buf[(p + 2)..].split('\n') {
        let (x, m, a, s) =
            sscanf::sscanf!(line, "{{x={},m={},a={},s={}}}", i64, i64, i64, i64).unwrap();
        ratings.push(Rating { x, m, a, s });
    }

    let val: i64 = ratings
        .iter()
        .filter(|x| system.does_accept(x))
        .map(|x| x.sum())
        .sum();
    println!("{}", val);

    let ranges = system.accept_ranges();
    let val: i64 = ranges.iter().map(|x| x.sum()).sum();
    println!("{}", val);
}

struct System {
    workflows: HashMap<String, Node>,
}

impl System {
    fn does_accept(&self, rating: &Rating) -> bool {
        let mut name = "in";
        while name != "A" && name != "R" {
            let mut node = self.workflows.get(name).unwrap();

            loop {
                // Trace branches
                match node {
                    Node::Branch {
                        condition,
                        left,
                        right,
                    } => {
                        let v = condition.variable;
                        let operand = condition.operand;
                        let operator = condition.operator;
                        let v = match v {
                            'x' => rating.x,
                            'm' => rating.m,
                            'a' => rating.a,
                            's' => rating.s,
                            _ => todo!(),
                        };
                        if (operator == '<' && v < operand) || (operator == '>' && v > operand) {
                            node = &*left;
                        } else {
                            node = &*right;
                        }
                    }
                    Node::Next(next_name) => {
                        name = next_name;
                        break;
                    }
                };
            }
        }
        name == "A"
    }

    fn accept_ranges(&self) -> Vec<RatingRange> {
        let range = RatingRange::new();
        let node = self.workflows.get("in").unwrap();

        let mut q = vec![(node, range)];
        let mut accepted = Vec::new();
        while !q.is_empty() {
            let (node, range) = q.pop().unwrap();
            match node {
                Node::Branch {
                    condition,
                    left,
                    right,
                } => {
                    let var = condition.variable;
                    let operand = condition.operand;
                    let operator = condition.operator;
                    let v = match var {
                        'x' => range.x,
                        'm' => range.m,
                        'a' => range.a,
                        's' => range.s,
                        _ => todo!(),
                    };
                    let mut lrange = range.clone();
                    let mut rrange = range.clone();
                    if operator == '<' {
                        let lv = (v.0, v.1.min(operand));
                        if lv.0 < lv.1 {
                            match var {
                                'x' => lrange.x = lv,
                                'm' => lrange.m = lv,
                                'a' => lrange.a = lv,
                                's' => lrange.s = lv,
                                _ => todo!(),
                            };
                            q.push((left, lrange));
                        }
                        let rv = (v.0.max(operand), v.1);
                        if rv.0 < rv.1 {
                            match var {
                                'x' => rrange.x = rv,
                                'm' => rrange.m = rv,
                                'a' => rrange.a = rv,
                                's' => rrange.s = rv,
                                _ => todo!(),
                            };
                            q.push((right, rrange));
                        }
                    } else {
                        let operand = operand + 1;
                        let lv = (v.0.max(operand), v.1);
                        if lv.0 < lv.1 {
                            match var {
                                'x' => lrange.x = lv,
                                'm' => lrange.m = lv,
                                'a' => lrange.a = lv,
                                's' => lrange.s = lv,
                                _ => todo!(),
                            };
                            q.push((left, lrange));
                        }
                        let rv = (v.0, v.1.min(operand));
                        if rv.0 < rv.1 {
                            match var {
                                'x' => rrange.x = rv,
                                'm' => rrange.m = rv,
                                'a' => rrange.a = rv,
                                's' => rrange.s = rv,
                                _ => todo!(),
                            };
                            q.push((right, rrange));
                        }
                    }
                }
                Node::Next(name) => {
                    if name == "A" {
                        accepted.push(range);
                    } else if name == "R" {
                    } else {
                        let node = self.workflows.get(name).unwrap().clone();
                        q.push((node, range));
                    }
                }
            };
        }
        accepted
    }
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let mut workflows = HashMap::new();
        for line in value.split('\n') {
            let tokens = Lexer::new(line).tokenize();
            let (name, flow) = Parser::new(tokens).parse();
            workflows.insert(name, flow);
        }
        System { workflows }
    }
}

#[derive(Debug)]
enum Node {
    Branch {
        condition: Condition,
        left: Box<Node>,
        right: Box<Node>,
    },
    Next(String),
}

#[derive(Debug)]
struct Condition {
    variable: char,
    operator: char,
    operand: i64,
}

struct Rating {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Rating {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Copy, Clone)]
struct RatingRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl RatingRange {
    fn new() -> RatingRange {
        RatingRange {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    fn sum(&self) -> i64 {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }
}

#[derive(Debug, Clone)]
enum Token {
    String(String),
    Number(i64),
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    Comma,
    Colon,
}

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.chars.peek() {
            Some(c) => match c {
                '{' => self.next_return_token(Token::LeftBrace),
                '}' => self.next_return_token(Token::RightBrace),
                ',' => self.next_return_token(Token::Comma),
                ':' => self.next_return_token(Token::Colon),
                '<' => self.next_return_token(Token::LessThan),
                '>' => self.next_return_token(Token::GreaterThan),
                c if c.is_numeric() => self.parse_number_token(),
                _ => self.parse_string_token(),
            },
            None => None,
        }
    }

    fn parse_number_token(&mut self) -> Option<Token> {
        let mut number_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() {
                self.chars.next();
                number_str.push(c);
            } else {
                break;
            }
        }

        if let Ok(number) = number_str.parse::<i64>() {
            Some(Token::Number(number))
        } else {
            None
        }
    }

    fn parse_string_token(&mut self) -> Option<Token> {
        let mut value = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
                self.chars.next();
                value.push(c);
            } else {
                break;
            }
        }
        Some(Token::String(value))
    }
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn parse(&mut self) -> (String, Node) {
        let name = match self.next() {
            Some(Token::String(x)) => x.clone(),
            _ => todo!(),
        };
        let workflow = self.parse_workflow();
        (name, workflow)
    }

    fn parse_workflow(&mut self) -> Node {
        self.next(); // Token '{'
        let node = self.parse_node();
        self.next(); // Token '}'
        node
    }

    fn parse_node(&mut self) -> Node {
        let token1 = self.next().unwrap();
        let token1 = match token1 {
            Token::String(x) => x.clone(),
            _ => todo!(),
        };

        let token2 = self.peek();
        match token2 {
            Some(Token::Comma) | Some(Token::RightBrace) | None => {
                return Node::Next(token1);
            }
            Some(Token::LessThan) | Some(Token::GreaterThan) => (),
            _ => todo!(),
        }
        let token2 = self.next().unwrap().clone();
        let token3 = self.next().unwrap().clone();
        let condition = match (token2, token3) {
            (Token::LessThan, Token::Number(val)) => Condition {
                variable: token1.chars().next().unwrap(),
                operator: '<',
                operand: val,
            },
            (Token::GreaterThan, Token::Number(val)) => Condition {
                variable: token1.chars().next().unwrap(),
                operator: '>',
                operand: val,
            },
            _ => todo!(),
        };
        self.next(); // Token ':'
        let left = self.parse_node();
        self.next(); // Token ','
        let right = self.parse_node();

        Node::Branch {
            condition,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }
}
