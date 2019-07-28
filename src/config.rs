use uinput::event::keyboard::Key;
use std::fs;

type TokensIter = std::iter::Peekable<std::vec::IntoIter<Token>>;
type BindingKey = i32;
type BindingValue = Vec<Key>;
type Bindings = std::collections::HashMap<BindingKey, BindingValue>;

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Assignment,
    NewLine,
    OpenBracket,
    CloseBracket,
    Word(String),
}

fn tokenize(s: &str) -> Vec<Token> {
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

fn parse_assignment(mut toks: &mut TokensIter, keys: &::std::collections::HashMap<String, Key>) -> (BindingKey, BindingValue) {
    let key = check(toks, Token::Word("assign".to_string()));
    check(toks, Token::Assignment);

    let mut values = Vec::new();

    loop {
        match toks.peek() {
            Some(Token::Word(w)) => {
                match keys.get(&w.to_lowercase()) {
                    Some(k) => values.push(k.clone()),
                    None => panic!("hmm {}", w)
                }

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


fn parse_section(mut toks: &mut TokensIter, keys: &::std::collections::HashMap<String, Key>) -> (String, Bindings) {
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
                let (k, v) = parse_assignment(&mut toks, keys);
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

pub fn parse(path: &str) -> std::collections::HashMap<String, Bindings> {
    let s = fs::read_to_string(path).unwrap();
    let mut toks = tokenize(&s).into_iter().peekable();
    let mut profiles = std::collections::HashMap::new();

    let mut keys = ::std::collections::HashMap::new();
    for k in Key::iter_variants() {
        keys.insert(format!("{:?}", k).to_lowercase(), k);
    }

    loop {
        match toks.peek() {
            Some(_) => {
                let (name, bindings) = parse_section(&mut toks, &keys);
                profiles.insert(name, bindings);
            },
            None => break
        };
    }

    profiles
}
