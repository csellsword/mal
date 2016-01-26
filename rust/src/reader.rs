use super::types::*;
use super::std::vec;
use super::regex as regex;

// I don't really understand this, can't find in docs but it is in the solution that came with
// the repo....
static token_regex: &'static str = r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"###;

struct Reader {
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn new(t: Vec<String>) -> Reader {
        Reader{tokens: t, pos: 0}
    }

    fn next(&mut self) -> Option<String> {
        self.pos = self.pos + 1;
        match self.tokens.get(self.pos -1) {
            Some(s) => Some(s.to_string()),
            _ => None,
        }
    }

    fn peek(&self) -> Option<String> {
        match self.tokens.get(self.pos) {
            Some(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

pub fn read_str(src: &str) -> MalType {
    let mut reader = Reader::new(tokenizer(src));
    read_form(&mut reader)
}

fn tokenizer(src: &str) -> Vec<String> {
    let re = regex::Regex::new(token_regex).unwrap();
    let mut v: Vec<String> = Vec::new();
    for t in re.captures_iter(src) {
        v.push(t.at(0).unwrap().to_string());
    }
    v
}

// strange, why mut twice?
fn read_form(mut reader: &mut Reader) -> MalType {
    println!("read_form");
    let tt = reader.next().unwrap();
    let t = tt.as_str();
    println!("read_form, {}", t);
    match t {
        "(" => read_list(&mut reader),
        _ => read_atom(&mut reader),
    }
}

fn read_list(reader: &mut Reader) -> MalType {
    println!("read_list");
    let mut result: Vec<MalType> = Vec::new();
    loop {
        let tt = reader.next().unwrap();
        let t = tt.as_str();
        println!("read_list, {}", t);
        match t {
            ")" => break,
            _ => result.push(read_form(reader)),
        }
    }
    MalType::Vector(result)
}

fn read_atom(reader: &mut Reader) -> MalType {
    let tok = reader.next().unwrap();
    let intReg = regex::Regex::new(r"^-?[0-9]+$").unwrap();
    if intReg.is_match(&tok) {
        MalType::Int(tok.parse().unwrap())
    }
    else {
        MalType::Sym(tok)
    }
}
