#[derive(Clone, Deserialize, Debug)]
pub struct Directive {
    pub name: String,
    pub command: String,

    #[serde(default)]
    pub iterator: Option<String>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Delimiter {
    pub open: String,
    pub close: String
}

#[derive(Clone, Deserialize, Debug)]
pub struct Rule {
    pub delimiter: Delimiter,
    pub directives: Vec<Directive>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Rules {
    pub key_regex: String,
    pub rules: Vec<Rule>
}
