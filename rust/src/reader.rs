use super::types::*;
use super::regex as regex;

// I don't really understand this, can't find in docs but it is in the solution that came with
// the repo....
static token_regex: &'static str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#;

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

pub fn read_str(src: &str) -> Result<MalType, String> {
    let mut reader = Reader::new(tokenizer(src));
    read_form(&mut reader)
}

fn tokenizer(src: &str) -> Vec<String> {
    let re = regex::Regex::new(token_regex).unwrap();
    let mut v: Vec<String> = Vec::new();
    for t in re.captures_iter(src) {
        let stage1 = t.at(0).unwrap().trim();
        if stage1 != "," {
            let test_str = stage1.trim_matches(',').trim().to_string();
						v.push(test_str);
        }
    }
    v
}

fn read_form(mut reader: &mut Reader) -> Result<MalType, String> {
    let tt = reader.peek().unwrap();
    let t = tt.as_str();
    match t {
        "(" => {
            reader.next();
            read_list(&mut reader)
        },
        _ => read_atom(&mut reader),
    }
}

fn read_list(reader: &mut Reader) -> Result<MalType, String> {
    let mut result: Vec<MalType> = Vec::new();
    loop {
        let tt = reader.peek();
        //let t = tt.as_str();
				let t = if let Some(s) = tt {
					s
				}
				else{
					return Err("Expected ')', got EOF".to_string());
				};
        match t.as_str() {
            ")" => {
                reader.next();
                break;
            },
            _ => {
							let form = try!(read_form(reader));
							result.push(form)
						},
        }
    }
    Ok(MalType::Vector(result))
}

fn read_atom(reader: &mut Reader) -> Result<MalType, String> {
    let tok = reader.next().unwrap();
    let int_reg = regex::Regex::new(r"^-?[0-9]+$").unwrap();
    if int_reg.is_match(&tok) {
        Ok(MalType::Int(tok.parse().unwrap()))
    }
    else {
        Ok(MalType::Sym(tok))
    }
}
