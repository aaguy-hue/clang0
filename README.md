# Clang0
**MAJOR NOTE: This is completely unrelated to the clang C compiler**, I just named
it this as a homage.

Anyways, this is a compiler for C0, a C-like programming language used for
Carnegie Mellon's introductory programming course 15-122 Principles of
Imperative Computation.

I am basing the concepts on a video series by the YouTube channel
(Premature Abstraction)[https://youtube.com/@PrematureAbstraction].


I am attempting to try and figure things out from scratch from reading about
the basic data structures and concepts without looking at other compilers, so
things will *probably* be inefficient and have issues, so please let me know
if you see any problems!!!

This is also my first time using Rust so let me know if there's any issues with
usage of the language here.



## TODO
- Read files
- Lexer
- Parser
- Transform to IR


## Some design decisions
If dynamic contract checking isn't enabled, then contracts are simply
interpreted as comments.

We will compile to LLVM IR rather than assembly. I may consider at some point
directly generating assembly for some architecture, but it seems somewhat
pointless for the sheer amount of effort to consider differences due to 
x86-64 extensions and whatnot.

I may implement optimizations like tail-end recursion at some point but idk
that's pretty far-fetched.

I think it would also be fun to do a basic sort of compile-time static contract
checking, would be a very fun technical challenge.



## Intended Usage

```bash
clang0 <inpfiles...>               // compile files to ./a.out executable
clang0 -d <inpfiles...>            // compile with dynamic contract checking
clang0 <filenames...> -o <outfile> // compiles to <outfile>
```


