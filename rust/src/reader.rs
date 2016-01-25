use super::types;
use super::std::vec;
use super::regex as regex;

// I don't really understand this, can't find in docs but it is in the solution that came with
// the repo....
static token_regex: &'static str = r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"###;

struct Reader {
    tokens: Vec<String>,
    pos: u64,
}

impl Reader {
    fn new(t: Vec<String>) -> Reader {
        Reader{tokens: t, pos: 0}
    }

    fn next(&mut self) -> Option<String> {
        self.pos = self.pos + 1;
        self.tokens.get(self.pos -1)
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos)
    }
}

fn read_str(src: &str) -> Reader {
    let reader = Reader::new(tokenizer(src));
    read_form(reader);
}

fn tokenizer(src: &str) -> Vec<String> {
    let re = regex::Regex::new(token_regex).unwrap();
    let mut v: Vec<String> = Vec::new();
    for t in re.captures_iter(src) {
        v.push(t);
    }
    v
}

fn read_form(reader: &mut Reader) -> MalType {
    let t = &reader.peek().unwrap();
    match t {
        '(' => {
            read_list(&mut reader);
        },
        _ => read_atom(&mut reader),
    }
}

fn read_list(reader: &mut Reader) {
    let mut result: Vec<MalType> = Vec::new();
    loop {
        let t = &reader.peek().unwrap();
        match t {
            ')' => break,
            _ => results.push(read_form(&reader)),
        }
    }
    MalType::Vector(result)
}

fn read_atom(reader: &mut Reader) {
    let tok = &reader.next().unwrap();
    let intReg = regex::Regex::new(r"^-?[0-9]+$").unwrap();
    if intReg.is_match(&tok) {
        MalType::Int(tok.parse().unwrap())
    }
    else {
        MalType::Sym(tok)
    }
}
