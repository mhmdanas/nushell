use crate::parser::lexer::SpannedToken;
use derive_new::new;
use getset::Getters;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Operator {
    pub fn print(&self) -> String {
        match *self {
            Operator::Equal => "==".to_string(),
            Operator::NotEqual => "!=".to_string(),
            Operator::LessThan => "<".to_string(),
            Operator::GreaterThan => ">".to_string(),
            Operator::LessThanOrEqual => "<=".to_string(),
            Operator::GreaterThanOrEqual => ">=".to_string(),
        }
    }
}

impl FromStr for Operator {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        match input {
            "==" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            "<" => Ok(Operator::LessThan),
            ">" => Ok(Operator::GreaterThan),
            "<=" => Ok(Operator::LessThanOrEqual),
            ">=" => Ok(Operator::GreaterThanOrEqual),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expression {
    Leaf(Leaf),
    Parenthesized(Box<Parenthesized>),
    Block(Box<Block>),
    Binary(Box<Binary>),
    Path(Box<Path>),
    VariableReference(Variable),
}

impl Expression {
    crate fn print(&self) -> String {
        match self {
            Expression::Leaf(l) => l.print(),
            Expression::Parenthesized(p) => p.print(),
            Expression::Block(b) => b.print(),
            Expression::VariableReference(r) => r.print(),
            Expression::Path(p) => p.print(),
            Expression::Binary(b) => b.print(),
        }
    }

    crate fn as_string(&self) -> Option<String> {
        match self {
            Expression::Leaf(Leaf::String(s)) => Some(s.to_string()),
            Expression::Leaf(Leaf::Bare(path)) => Some(path.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, new)]
pub struct Block {
    crate expr: Expression,
}

impl Block {
    fn print(&self) -> String {
        format!("{{ {} }}", self.expr.print())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, new)]
pub struct Parenthesized {
    crate expr: Expression,
}

impl Parenthesized {
    fn print(&self) -> String {
        format!("({})", self.expr.print())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Getters, new)]
pub struct Path {
    #[get = "crate"]
    head: Expression,

    #[get = "crate"]
    tail: Vec<String>,
}

impl Path {
    crate fn print(&self) -> String {
        let mut out = self.head.print();

        for item in self.tail.iter() {
            out.push_str(&format!(".{}", item));
        }

        out
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Variable {
    It,
    True,
    False,
    Other(String),
}

impl Variable {
    fn print(&self) -> String {
        match self {
            Variable::It => format!("$it"),
            Variable::True => format!("$true"),
            Variable::False => format!("$false"),
            Variable::Other(s) => format!("${}", s),
        }
    }
}

impl FromStr for Variable {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(match input {
            "it" => Variable::It,
            "true" => Variable::True,
            "false" => Variable::False,
            other => Variable::Other(other.to_string()),
        })
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct BarePath {
    head: String,
    tail: Vec<String>,
}

impl BarePath {
    crate fn from_tokens(head: SpannedToken, tail: Vec<SpannedToken>) -> BarePath {
        BarePath {
            head: head.to_string(),
            tail: tail.iter().map(|i| i.to_string()).collect(),
        }
    }

    crate fn to_string(&self) -> String {
        bare_string(&self.head, &self.tail)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Leaf {
    String(String),
    Bare(BarePath),

    #[allow(unused)]
    Boolean(bool),
    Int(i64),
}

crate fn bare_string(head: &String, tail: &Vec<String>) -> String {
    let mut out = vec![head.clone()];
    out.extend(tail.clone());
    itertools::join(out, ".")
}

impl Leaf {
    fn print(&self) -> String {
        match self {
            Leaf::String(s) => format!("{:?}", s),
            Leaf::Bare(path) => format!("{}", path.to_string()),
            Leaf::Boolean(b) => format!("{}", b),
            Leaf::Int(i) => format!("{}", i),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, new)]
pub struct Binary {
    crate left: Expression,
    crate operator: Operator,
    crate right: Expression,
}

impl Binary {
    fn print(&self) -> String {
        format!(
            "{} {} {}",
            self.left.print(),
            self.operator.print(),
            self.right.print()
        )
    }
}

#[derive(Debug, Clone)]
pub enum Flag {
    Shorthand(String),
    Longhand(String),
}

impl Flag {
    #[allow(unused)]
    fn print(&self) -> String {
        match self {
            Flag::Shorthand(s) => format!("-{}", s),
            Flag::Longhand(s) => format!("--{}", s),
        }
    }
}

#[derive(new, Debug, Clone)]
pub struct ParsedCommand {
    crate name: String,
    crate args: Vec<Expression>,
}

#[derive(new, Debug)]
pub struct Pipeline {
    crate commands: Vec<ParsedCommand>,
}

impl Pipeline {
    crate fn from_parts(command: ParsedCommand, rest: Vec<ParsedCommand>) -> Pipeline {
        let mut commands = vec![command];
        commands.extend(rest);

        Pipeline { commands }
    }
}