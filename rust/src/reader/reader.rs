extern crate regex;

mod reader {
    use std::vec;

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
            self.pos = pos + 1;
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
        let mut v: Vector<String> = Vector::new();
        for t in re.captures_iter(src) {
            v.push(t);
        }
        v
    }

    fn read_form(reader: &Reader) {
        let t = &reader.peek().unwrap();
        match t {
            '(' => {
                read_list(&reader);
            },
            _ => read_atom(&reader),

        }
    }
}
