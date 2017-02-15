RCC
===

`rcc` is meant to be a compiler for reversible programming languages. Some examples of such languages are [Janus](https://en.wikipedia.org/wiki/Janus_(time-reversible_computing_programming_language)), [rFun](http://topps.diku.dk/pirc/?id=rfun), and my own language, Rever. The compiler *will* support compiling reversible languages for irreversible architectures eventually, but for now it's easier to target a reversible architecture, like [REL](https://github.com/1011X/REL-16). Other reversible architectures will be supported too, of course, like [Pendulum](https://dspace.mit.edu/bitstream/handle/1721.1/36039/33342527-MIT.pdf?sequence=2).

Currently, the compiler is no more than just a set of incomplete parsers that build ASTs for Rever, Janus, and Extended Janus. After the Rever parser is done, I'll start writing the code generator for REL assembly instructions.

So basically, the plan is:
1. Parser for Rever
2. Code generator for REL
3. Parser for other languages
4. Code generator for other (reversible) architectures
5. Hook-up to LLVM to generate code for irreversible architectures
