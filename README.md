# Rapit

A simple programming language, for fun.

## Code Sample

The compiler in its current position can parse the following into an AST:

```

                }

```

## Design Decisions

### Compiling
3 steps of compiling

1. Lexing
2. Parsing
3. Code-Gen

Everything will be transpiled into C++ for now.

Target will be C++ 20. Will be relying heavily on `auto` keyword for now.

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

```
program = { statement }-

statement = "if" paren_expr statement |
            "fn" ident paren_ident statement |
            "let" ident "=" expr |
            "{" { statement } "}" |
            expr ";"

paren_ident = "(" { ident[,] } ")"

paren_expr = "(" expr ")"

expr = term | 
    expr '+' expr | 
    expr '-' expr | 
    expr '<' expr

term = ident | literal | paren_expr

ident = { "a".."z" }-
literal = { "0".."9" }-

```

### Language Features
No for loops. Only recursion.

Only top level members are functions.