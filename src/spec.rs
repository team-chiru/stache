pub struct Spec {
    pub tests: Vec<Test>
}

impl Spec {
    pub fn new(spec_file: String) -> Self {
        unimplemented!()
    }
}

pub struct Test {
    pub name: String,
    pub desc: String,
    pub data: String,
    pub template: String,
}
