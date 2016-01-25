use types;

mod printer {
    fn pr_str(datum: &MalType) -> String {
        match datum {
            Int(i) => i.to_string(),
            Sym(s) => s,
            Vector(v) => {
                let mut stuff = String::from("(");
                for thing in &v {
                    stuff.push_str(&pr_str(thing));
                    stuff.push_str(" ");
                }
                stuff.pop();
                stuff.push_str(")");
                stuff
            },
            _ => panic!("pr_str not implemented for type: {}", datum.typename()),
        }
    }
}
