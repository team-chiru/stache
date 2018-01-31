extern crate mustache;
use mustache::Mustache;

extern crate stache;
use stache::{ TemplateEngine, TemplateCompiler };

extern crate serde_json;

fn main() {
    let template = Mustache::compiles_template(String::from(r#"
{{#root}}
<TITLE>{{name}}</TITLE>
<H1>{{description}}</H1>
{{/root}}
{{^root}}
<DT><H3>{{name}}</H3>
<DD>{{description}}
{{/root}}
<DL><p>
{{#links}}
    <DT><A HREF={{url}}>{{name}}</A>
{{/links}}
</DL><p>
    "#)).unwrap();

    let data1 = serde_json::from_str(r#"
{
    "root": true,
    "name": "John Doe",
    "description": "This is John Doe",
    "links": [
        {
            "name": "Facebook",
            "url": "https://www.facebook.com/"
        },
        {
            "name": "Twitter",
            "url": "https://www.twitter.com/"
        }
    ]
}
    "#).unwrap();

    let data2 = serde_json::from_str(r#"
{
    "name": "John Doe",
    "description": "This is John Doe",
    "links": [
        {
            "name": "Facebook",
            "url": "https://www.facebook.com/"
        },
        {
            "name": "Twitter",
            "url": "https://www.twitter.com/"
        }
    ]
}
    "#).unwrap();

    println!("{}", Mustache::render_once(template.clone(), vec![data1]).unwrap());
    println!("{}", Mustache::render_once(template.clone(), vec![data2]).unwrap());
}