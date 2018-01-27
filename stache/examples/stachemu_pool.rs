fn main() {
    let base = String::from("specs/stachemu/");
    let path = base + "interpolation.yml";
    let mut pool = StachemuPool::default();

    pool.path(&path);
    pool.name("Dotted Names - Complex Interpolation");

    let (template, partials, data) = pool.debug().unwrap();
    let result = Stachemu::render(template, partials, vec![data]).unwrap();
    println!("{:?}", result);
}
