# Rapit

A simple programming language, for fun.

## Design Decisions

### Compiling
3 steps of compiling

1. Lexing
2. Parsing
3. Code-Gen

Everything will be transpiled into C for now.

### EBNF for parsing

Note that this is not correct yet.

```
program ::= fn_decl+

statement ::= fn_decl | 

fn_decl ::= 'fn' ident OPEN_PAREN param_list CLOSE_PAREN OPEN_BRACKET expression+ CLOSE_BRACKET 

fn_call ::= ident OPEN_PAREN argument_list CLOSE_PAREN

if_decl ::= 'if' OPEN_PAREN argument_list CLOSE_PAREN OPEN_BRACKET expression+ CLOSE_BRACKET

argument_list ::= (expression ',')*

assignment ::= ident EQUALS expression

expression ::= fn_call | if_decl | ident | bin_op | literal

bin_op :: = expression OPERATOR expression

primary ::= '(' expression ')' | NUMBER

ident ::= STRING

param_list ::= (ident ',')*

literal ::= '"' STRING '"' | NUMBER

STRING ::= [a-z]*
NUMBER ::= [0-9]*

OPERATOR ::= '+' | '-' | '*' | '/'

EQUALS ::= '='

OPEN_PAREN ::= '('

CLOSE_PAREN ::= ')'

OPEN_BRACKET ::= '{'

CLOSE_BRACKET ::= '}'

WHITESPACE = ' ' | '\n'
```

### Language Features
No for loops. Only recursion.