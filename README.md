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


| Usage | Notation |
| --- | --- |
definition |	=
concatenation | ,
termination | ;
alternation | \| |
optional | [ ... ] |
repetition | { ... } |
grouping | ( ... ) |
terminal string | " ... " |
terminal string	| ' ... ' |
comment | (* ... *) |
special sequence | ? ... ? |
exception | - |

Note that this is not correct yet.

```
program = { statement }-

statement = "if" paren_expr statement |
            "fn" ident paren_ident statement |
            "{" { statement } "}" |
            expr ";"

paren_ident = "(" { ident } ")"

paren_expr = "(" expr ")"

expr = test | ident "=" expr

test = sum | sum "<" sum

sum = term | sum "+" sum | sum "-" sum

term = ident | literal | paren_expr

ident = { "a".."z" }-
literal = { "0".."9" }-

```

### Language Features
No for loops. Only recursion.

Only top level members are functions.