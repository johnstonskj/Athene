/*
Language: OWL Functional-Style Syntax
Website: https://www.w3.org/TR/owl2-syntax/
Category: idl
*/

function hljsOwl(hljs) {

    const QNAME = /([a-z])?:[a-z]/;

    const STRING = hljs.inherit(hljs.QUOTE_STRING_MODE, { illegal: null });

    const IRI_RE = /<[^>]+>/;

    const IRI = {
        scope: 'string',
        begin: IRI_RE,
    };

    const LANGUAGE_STRING = hljs.inherit(hljs.QUOTE_STRING_MODE, { end: /@[a-z]/ });

    const TYPED_STRING = hljs.inherit(hljs.QUOTE_STRING_MODE, { end: /\^\^(<[^>]+>)|(([a-z])?:[a-z])/ });

    const LITERAL = {
        variants: [
            STRING,
            LANGUAGE_STRING,
            TYPED_STRING,
            IRI
        ]
    };

    const COMMENT = hljs.COMMENT(
        '#', '$',
        { relevance: 0 }
    );

    const EQUAL_FOR_PREFIX = {
        scope: 'operator',
        begin: '='
    }

    const OWL_FUNCTION_IDENT = /([A-Z][a-z]+)+/;

    const OWL_FUNCTION = {
        scope: 'title.function',
        begin: OWL_FUNCTION_IDENT + /\s*\(\s*/,
        end: /\s*\)/,
        contains: [
            COMMENT,
            QNAME,
            EQUAL_FOR_PREFIX,
            LITERAL
        ]
    };

    return {
        name: 'Owl',
        aliases: ['owl'],
        illegal: /\S/,
        contains: [
            COMMENT,
            OWL_FUNCTION,
        ]
    };
}