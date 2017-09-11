pub type Symbol = String;
pub type Delimiter = (String, String);

pub struct Expression {
    pub delimiters: Vec<Delimiter>,
    pub directives: Vec<Symbol>,
    pub key_regex: String
}
