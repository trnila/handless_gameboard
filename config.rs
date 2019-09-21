use uinput::event::keyboard::Key;

use std::fs::File;
use std::time::Duration;


use std::{thread, time};

use std::io::prelude::*;
use serial::prelude::*;
use std::fmt;
use std::fs;



#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Assignment,
    NewLine,
    OpenBracket,
    CloseBracket,
    Word(String),
}

fn tokenize() -> Vec<Token> {
    let s = fs::read_to_string("sam2.txt").unwrap();

    let mut toks = Vec::new();
    let mut word = String::from("");
    let mut line = 1;
    let mut col = 1;

    for b in s.chars() {
        if b.is_alphanumeric() {
            word.push(b);
        } else {
            if word.len() > 0 {
                toks.push(Token::Word(word));
                word = String::from("");
            }
            
            let t = match b {
                '=' => Some(Token::Assignment),
                '[' => Some(Token::OpenBracket),
                ']' => Some(Token::CloseBracket),
                ' ' | '\t' => None,
                '\n' => {
                    line += 1;
                    col = 0;
                    Some(Token::NewLine)
                },
                _ => panic!("Unknown char: {} on line {} column {}", b, line, col)
            };

            if let Some(token) = t {
                toks.push(token);
            }
        }
    }

    toks
}

type TokensIter = std::iter::Peekable<std::vec::IntoIter<Token>>;
type BindingKey = i32;
type BindingValue = Vec<String>;
type Bindings = std::collections::HashMap<BindingKey, BindingValue>;

fn check(toks: &mut TokensIter, expected: Token) -> Token {
    if let Some(got) = toks.next() {
        if std::mem::discriminant(&got) != std::mem::discriminant(&expected) {
            panic!("expected {:?} but got {:?}", expected, got);
        }
        got
    } else {
        panic!("expected {:?} but end of file occured", expected)
    }
}

fn parse_assignment(mut toks: &mut TokensIter) -> (BindingKey, BindingValue) {
    let key = check(toks, Token::Word("assign".to_string()));
    check(toks, Token::Assignment);

    let mut values = Vec::new();

    loop {
        match toks.peek() {
            Some(Token::Word(w)) => {
                values.push(w.clone());
                toks.next();
            },
            Some(Token::NewLine) => {
                toks.next();
                break;
            }
            _ => panic!("unexpected")
        };
    }    
    // optionally skip newlines
    loop {
        match toks.peek() {
            Some(Token::NewLine) => toks.next(),
            _ => break
        };
    }

//    check(toks, Token::NewLine);
    println!("ll:{:?} {:?}", key, values);


    let key:i32 = match key {
        Token::Word(w) => w.parse().unwrap(),
        _ => -1
    };


    (key, values)
}


fn parse_section(mut toks: &mut TokensIter) -> (String, Bindings) {
    check(toks, Token::OpenBracket);
    let section = check(toks, Token::Word("ksec".to_string()));
    check(toks, Token::CloseBracket);

    // optionally skip newlines
    loop {
        match toks.peek() {
            Some(Token::NewLine) => toks.next(),
            _ => break
        };
    }

    let mut bindings = std::collections::HashMap::new();

    // assignments
    loop {
        match toks.peek() {
            Some(Token::OpenBracket) => break,
            Some(_) => {
                let (k, v) = parse_assignment(&mut toks);
                bindings.insert(k, v);
            },
            None => break
        };
    }

    let section = match section {
        Token::Word(s) => s,
        _ => "".to_string()
    };
    (section, bindings)
}

pub fn parse() -> std::collections::HashMap<String, Bindings> {
    let mut toks = tokenize().into_iter().peekable();
    let mut profiles = std::collections::HashMap::new();

    let mut keys = ::std::collections::HashMap::new();
    for k in Key::iter_variants() {
        keys.insert(format!("{:?}", k).to_lowercase(), k);
    }

    loop {
        match toks.peek() {
            Some(_) => {
                let (name, bindings) = parse_section(&mut toks);
                profiles.insert(name, bindings);
            },
            None => break
        };
    }

    profiles
    /*
    let mut mapping = ::std::collections::HashMap::new();

    let mut key = 0;
    let mut in_key = true;
    println!("tok: {:?}", tokenize());

    for tok in tokenize() {
        match tok {
            Token::NewLine => in_key = true,
            Token::Colon => {},
            Token::Word(w) => {
                if in_key {
                    in_key = false;
                    key = w.parse::<i32>().unwrap();
                    println!("{}", key);
                } else {
                    match keys.get(&w.to_lowercase()) {
                        Some(k) => mapping.entry(key).or_insert(Vec::new()).push(k),
                        None => println!("key {} doesnt exists!", w)
                    }
                }
            }
        }
    }

    println!("{:?}", mapping);*/
}
