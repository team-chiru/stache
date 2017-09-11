pub type Command = String;

#[derive(Clone, Deserialize, Debug)]
pub struct Directive {
    pub name: String,
    pub command: Command,

    #[serde(default)]
    pub iterator: Option<String>
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub struct Delimiter {
    pub open: String,
    pub close: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct RuleHeap {
    pub delimiter: Delimiter,
    pub directives: Vec<Directive>
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub struct Rule {
    pub name: String,
    pub command: Command,
    pub delimiter: Delimiter,
    pub iterator: Option<String>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Description {
    pub key_regex: String,
    pub rules: Vec<RuleHeap>
}

impl Description {
    pub fn find(&self, del: Delimiter, cmd: Command) -> Option<Rule> {
        for &RuleHeap { delimiter: ref d, directives: ref ds } in &self.rules {
            for &Directive { name: ref n, command: ref c, iterator: ref i } in ds {
                if del == *d && cmd == *c {
                    return Some(
                        Rule {
                            name: n.clone(),
                            command: cmd,
                            delimiter: del,
                            iterator: i.clone()
                        }
                    )
                }
            }
        }

        None
    }
}

pub struct Expression {
    pub delimiters: Vec<Delimiter>,
    pub commands: Vec<Command>,
    pub key_regex: String
}

pub trait DescriptionCollector where Self: Sized {
    fn collect(&Description) -> Vec<Self>;
}

impl DescriptionCollector for Command {
    fn collect(descr: &Description) -> Vec<Command> {
        let mut v: Vec<Self> = Vec::new();

        for &RuleHeap { directives: ref ds, .. } in &descr.rules {
            for &Directive { command: ref c, .. } in ds {
                if !v.contains(c) {
                    v.push(c.clone());
                }
            }
        }

        v
    }
}

impl DescriptionCollector for Delimiter {
    fn collect(descr: &Description) -> Vec<Delimiter> {
        let mut v: Vec<Self> = Vec::new();

        for &RuleHeap { delimiter: ref del, .. } in &descr.rules {
            if !v.contains(del) {
                v.push(del.clone());
            }
        }

        v
    }
}

impl DescriptionCollector for Rule {
    fn collect(descr: &Description) -> Vec<Rule> {
        use std::collections::HashSet;
        let mut h: HashSet<Self> = HashSet::new();

        for rh in &descr.rules {
            for d in &rh.directives {
                h.insert(Rule {
                    name: d.name.clone(),
                    command: d.command.clone(),
                    delimiter: rh.delimiter.clone(),
                    iterator: d.iterator.clone()
                });
            }
        }

        h.into_iter().collect::<Vec<Self>>()
    }
}

impl DescriptionCollector for Expression {
    fn collect(d: &Description) -> Vec<Expression> {
        vec![
            Expression {
                delimiters: Delimiter::collect(d),
                commands: Command::collect(d),
                key_regex: d.key_regex.clone()
            }
        ]
    }
}
