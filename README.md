# stachemu
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fteam-chiru%2Fstachemu.svg?type=shield)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fteam-chiru%2Fstachemu?ref=badge_shield)

Stachemu uses a super-set of the [Mustache][1] templating syntax in order to build data structures.

> Verlan is an argot in the French language, featuring inversion of syllables in a word, and is common in slang and youth language. [Wikipedia][2]

## motivation
Stachemu can be used to easily build and match templatized strings from data format.

## example
from this template:
```html
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
{{/Node}}
```

this html content:
```html
<TITLE>root</TITLE>
<H1>description of root</H1>
<DL><p>
   <DT><H3>node 1</H3>
   <DL><p>
      <DT><A HREF="url">link 1</A>
   </DL><p>
</DL><p>
```

will become the following raw structure:
```js
{
   Node: {
      name: "root",
      description: "description of root",
      root: true,
      nodes: [
         Node: {
            name: "node 1",
            links: [
               Link: {
                  name: "link 1",
                  url: "url"
               }
            ]
         }
      ]
   }
}
```
and *vice versa* with a compliant mustache renderer.

[1]: https://mustache.github.io/
[2]: https://en.wikipedia.org/wiki/Verlan

# license
see [license file](https://github.com/team-chiru/stachemu/blob/master/LICENSE)


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fteam-chiru%2Fstachemu.svg?type=large)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fteam-chiru%2Fstachemu?ref=badge_large)