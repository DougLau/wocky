A tool to analyze and generate text

Installation:
```shell
cargo install wocky
```

### Lexicon

The `lx` sub-command lists words from the built-in lexicon.

- With no options, all entries are listed
- The `-f` option lists all known word forms
- The `-c` option filters words by class.  Provide a comma-separated list to
  specify classes:

Option | Word Class
-------|-----------
`A`    | Adjective
`Av`   | Adverb
`C`    | Conjunction
`D`    | Determiner
`I`    | Interjection
`N`    | Noun
`P`    | Preposition
`Pn`   | Pronoun
`V`    | Verb

### Reading a Text

The `rd` sub-command reads UTF-8 text from `stdin`, which can be redirected
from a file.

The `-h` switch outputs the same text highlighted by word class.

Without any options, a summary of token kinds is listed:

```
> wocky rd < Dr_Jeckyll_And_Mr_Hyde.txt

 3916 l Lexicon
    1 f Foreign
    4 o Ordinal
    0 r Roman
    2 n Number
   12 a Acronym
   37 p Proper
   16 s Symbol
    6 u Unknown
```

Comma-separated options can be added to list all tokens of a kind.

Option | Kind    | Description
-------|---------|--------------------------
`l`    | Lexicon | Found in built-in lexicon
`f`    | Foreign | Foreign words (non-English)
`o`    | Ordinal | Ordinal numbers (1st, 2nd, etc.)
`r`    | Roman   | Roman numerals (IV, LXI, etc.)
`n`    | Number  | Other words containing numbers
`a`    | Acronym | Acronyms / initialisms (ALL-CAPS)
`p`    | Proper  | Proper names / nouns
`s`    | Symbol  | Symbols / letters
`u`    | Unknown | Unknown (no other kind)
`A`    | All     | All kinds

## Word Lookup

If a word is provided instead of a sub-command, all entries matching that
word in the lexicon will be printed.
