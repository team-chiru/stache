pub type Symbol<'s> = &'s str;
pub type Delimiter<'d> = (&'d str, &'d str);

pub struct Expression<'e> {
    pub delimiters: Vec<Delimiter<'e>>,
    pub directives: Vec<Symbol<'e>>,
    pub keySymbols: &'e str
}
