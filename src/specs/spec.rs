#[derive(Deserialize, Debug)]
pub struct Spec<Test> {
    pub tests: Vec<Test>
}

#[derive(Clone, Deserialize, Debug)]
pub struct Test<Input, Output> {
    pub name: String,
    pub desc: String,
    pub data: Input,
    pub template: String,
    pub expected: Output,
}
