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

fn_decl ::= 'fn' ident OPEN_PAREN param_list CLOSE_PAREN OPEN_BRACKET expr+ CLOSE_BRACKET 

fn_call ::= ident OPEN_PAREN argument_list CLOSE_PAREN

param_list ::= (ident ',')*

argument_list ::= (expr ',')*

assignment ::= ident EQUALS expr

expr ::= fn_call | ident | ident OPERATOR expr | expr OPERATOR expr

ident ::= STRING

literal ::= '"' STRING '"' | NUMBER

STRING ::= [a-z]*
NUMBER ::= [0-9]*

OPERATOR ::= '+' | '-' | '*' | '/'

EQUALS ::= '='

OPEN_PAREN ::= '('

CLOSE_PAREN ::= ')'

OPEN_BRACKET ::= '{'

CLOSE_BRACKET ::= '}'
```

### Language Features
No for loops. Only recursion.