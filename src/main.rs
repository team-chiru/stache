extern crate stachemu;

use stachemu::compiler;

fn main() {
    let rules = compiler::compile("
{{=Node}}
   {{#root}}
      <TITLE>{{name}}</TITLE>
      <H1>{{description}}</H1>
   {{/root}}
   {{^root}}
      <DT><H3{{?...}}>{{name}}</H3>
      <DD>{{?description}}
   {{/root}}
   <DL><p>
      {{#links}}
         {{=Link}}
            <DT><A HREF={{url}} {{?...}}>{{name}}</A>
         {{/Link}}
      {{/links}}
      {{#nodes}}
         {{>Node}}
      {{/nodes}}
   </DL><p>
{{/Node}}".to_string()).unwrap();

    println!("{:?}", rules);
}
