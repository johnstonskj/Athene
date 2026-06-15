/*
Language: Turtle
Website: https://www.w3.org/TR/turtle/
Category: idl
*/

function hljsTurtle(hljs) {
  const IDENT_RE = /[a-zA-Z_][a-zA-Z0-9_:]*/;
  const KEYWORDS =
    'prefix base a';
  return {
    name: 'Turtle',
    aliases: ['turtle', 'ttl'],
    keywords: {
      keyword: KEYWORDS,
    },
    contains: [
      {
        className: 'comment',
        begin: /#/,
        end: /$/
      },
      {
        className: 'string',
        variants: [
          hljs.QUOTE_STRING_MODE,
          hljs.inherit(hljs.QUOTE_STRING_MODE, {
            end: /@[a-z]/
          }),
          hljs.inherit(hljs.QUOTE_STRING_MODE, {
            end: /\^\^(<[^>]+>)|([a-zA-Z_][a-zA-Z0-9_:]*)/
          })
        ]
      },
      {
        className: 'number',
        begin: /\b(\d[\d_]*(\.[0-9_]+)?([eE][+-]?[0-9_]+)?)/,
        relevance: 0
      },
      // Known prefixes
      {
        className: 'title',
        begin: /(rdf|rdfs|xsd|owl):([a-zA-Z_]+[a-zA-Z0-9_-]*)?/
      },
      // URI
      {
        className: 'string',
        begin: /<[^>]+>/
      },
    ]
  };
}
