#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

#[derive(Debug)]
enum TokenType {
    Symbol,
    Number,
    Punctuation,
}

#[derive(Debug)]
enum Punctuation {
    Dot,
    Comma,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Colon,
    Tilde,
    ExclamationMark,
    QuestionMark,
    UnderScore,
    Hashtag,
    LargerThan,
    SmallerThan,
    Plus,
    Minus,
    SingleQuote,
    DoubleQuote,
    Equals,
    Slash,
    Backslash,
    Tick,
    Ampersand,
    Bar,
    Star,
}

const PUNCTS: [char; 28] = [
    '.',
    ',',
    '(',
    ')',
    '[',
    ']',
    '{',
    '}',
    ';',
    ':',
    '~',
    '!',
    '?',
    '_',
    '#',
    '>',
    '<',
    '+',
    '-',
    '\'',
    '\"',
    '=',
    '/',
    '\\',
    '`',
    '&',
    '|',
    '*', 
];

fn is_punct(c: char) -> bool {
    for p in PUNCTS {
        if c == p {
            return true;
        }
    }
    return false;
}

#[derive(Debug)]
enum TokenValue {
    Number(usize),
    Symbol(String),
    Punct(Punctuation),
}

#[derive(Debug)]
struct Token {
    t_type: TokenType,
    t_value: TokenValue,
}

/*
tokens.push(Token {
    t_type: TokenType::Integer, 
    t_value: TokenValue {v_int: 69}
});
*/

const WHITESPACE: [char; 3] = [
    ' ',
    '\n',
    '\t',
];

fn is_whitespace(c: char) -> bool {
    return WHITESPACE.iter().any(|&cc| cc == c);
}

fn get_token(string: &String, index: &mut usize) -> String {
    let mut result: String = "".to_string();
    while *index < string.len() && is_whitespace(string.chars().nth(*index).unwrap()) {
        *index += 1;
    }

    if *index < string.len() && is_punct(string.chars().nth(*index).unwrap()) {
        result.push(string.chars().nth(*index).unwrap());
        *index += 1;
        return result;
    }

    while *index < string.len() && !is_punct(string.chars().nth(*index).unwrap()) && !is_whitespace(string.chars().nth(*index).unwrap()) {
        result.push(string.chars().nth(*index).unwrap());
        *index += 1;
    }
    return result;
}

fn tokenize(string: String, tokens: &mut Vec<Token>) {
    let mut index: usize = 0;
    while is_whitespace(string.chars().nth(index).unwrap()) && index < string.len() {
        index += 1;
    }
    let mut tokenstrings: Vec<String> = vec![];
    while index < string.len() {
        let tokenstring: String = get_token(&string, &mut index);
        tokenstrings.push(tokenstring);
    }

    for ts in tokenstrings {
        if ts.len() == 1 && is_punct(ts.chars().nth(0).unwrap()) {
            let punct_type: Punctuation = match ts.chars().nth(0).unwrap() {
                '.' => Punctuation::Dot,
                ',' => Punctuation::Comma,
                '(' => Punctuation::OpenParen,
                ')' => Punctuation::CloseParen,
                '[' => Punctuation::OpenBracket,
                ']' => Punctuation::CloseBracket,
                '{' => Punctuation::OpenBrace,
                '}' => Punctuation::CloseBrace,
                ';' => Punctuation::Semicolon,
                ':' => Punctuation::Colon,
                '~' => Punctuation::Tilde,
                '!' => Punctuation::ExclamationMark,
                '?' => Punctuation::QuestionMark,
                '_' => Punctuation::UnderScore,
                '#' => Punctuation::Hashtag,
                '>' => Punctuation::LargerThan,
                '<' => Punctuation::SmallerThan,
                '+' => Punctuation::Plus,
                '-' => Punctuation::Minus,
                '\'' => Punctuation::SingleQuote,
                '\"' => Punctuation::DoubleQuote,
                '=' => Punctuation::Equals,
                '/' => Punctuation::Slash,
                '\\' => Punctuation::Backslash,
                '`' => Punctuation::Tick,
                '&' => Punctuation::Ampersand,
                '|' => Punctuation::Bar,
                '*' => Punctuation::Star,
                _ => panic!("How the fuck")
            };

            tokens.push( Token {
                t_type: TokenType::Punctuation,
                t_value: TokenValue::Punct(punct_type),
            })
        } else if ts.parse::<usize>().is_ok() {
            tokens.push( Token {
                t_type: TokenType::Number,
                t_value: TokenValue::Number(ts.parse::<usize>().unwrap())
            })
        } else {
            if ts.len() == 0 {
                continue;
            }
            tokens.push( Token {
                t_type: TokenType::Symbol,
                t_value: TokenValue::Symbol(ts),
            })
        }
    }
}

/* 
funny memory leak 
fn main() {
    let foo: String = "bar".to_string();
    drop(foo.leak());
}
*/


fn main() -> std::io::Result<()>{
    let mut args = std::env::args();
    let mut tokens: Vec<Token> = vec![];
    let runpath: String = args.next().unwrap();
    let filename: String = args.next().unwrap();
    
    let filestring = fs::read_to_string(filename)?;

    tokenize(filestring, &mut tokens);
    for ts in tokens {
        match ts.t_value {
            TokenValue::Number(value) => println!("Number: {value}"),
            TokenValue::Punct(value) => println!("Punctuation: {:?}", value),
            TokenValue::Symbol(value) => println!("Symbol: \"{value}\""),
        }
    }
    Ok(())
}
