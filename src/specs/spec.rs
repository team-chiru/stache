#[derive(Deserialize, Debug)]
pub struct Spec<I,O> {
    pub tests: Vec<Test<I,O>>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Test<I,O> {
    pub name: String,
    pub desc: String,
    pub data: I,
    pub template: String,
    pub expected: O,
}
