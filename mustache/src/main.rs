extern crate mustache;
use mustache::Mustache;

extern crate stache;
use stache::{ file, TemplateCompiler };

fn main() {
    println!("{:?}", Mustache::compiles_template(
        String::from(r#"
{{#root}}<TITLE>{{name}}</TITLE><H1>{{description}}</H1>{{/root}}
{{^root}}<DT><H3>{{name}}</H3><DD>{{description}}{{/root}}
<DL><p>
{{#links}}
    <DT><A HREF={{url.name}}>{{name}}</A>
{{/links}}
{{#nodes}}
    {{>Node}}
{{/nodes}}
</DL><p>
        "#)
    ));
}
