RCC
===

RCC is a compiler for reversible programming languages. Some examples of such languages are [Janus](https://en.wikipedia.org/wiki/Janus_(time-reversible_computing_programming_language)), [rFun](http://topps.diku.dk/pirc/?id=rfun), and my own language, Rever. For now the plan is to compile these languages to a reversible architecture I've made, called [REL](https://github.com/1011X/REL-16). Later I'll add support for other reversible architectures, like [Bob](https://link.springer.com/chapter/10.1007/978-3-642-29517-1_3).

Currently, RCC can parse source for Rever and an improved version of Janus into ASTs. I've also started writing the code generator to compile source code to REL assembly instructions.

The code doesn't do much right now; it only parses a test string and prints the AST, but feel free to mess around with it if it peaks your interest.