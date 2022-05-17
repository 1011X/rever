Rever
=====

Rever is an experimental programming language trying out different ideas at the same time. These ideas include:
* reversible computation
* significant newlines
* [bijective numerals]
* [predictive parser]
* procedures *and* functions
* [copy-in copy-out] semantics

Its syntax is inspired by [Janus], Pascal, Rust, and Haskell. It's in the same syntax family as Crunch and similar languages.

[Janus]: https://en.wikipedia.org/wiki/Janus_(time-reversible_computing_programming_language)
[predictive parser]: https://en.wikipedia.org/wiki/Predictive_parser
[bijective numerals]: https://en.wikipedia.org/wiki/Bijective_numeration
<!-- [Bob](https://link.springer.com/chapter/10.1007/978-3-642-29517-1_3). -->


Installation
------------

First, [install Rust]. Then, either download the ZIP file for this project and unzip it, or `git clone` this project, whichever you're most comfortable with.

Once that's done, you can start playing around with the REPL by running

    cargo run

As of now, the interpreter can only do math, define procedures, and print to the terminal. Try some of the examples in the `examples/` folder!

[install Rust]: https://www.rust-lang.org/tools/install


What is reversible computing?
-----------------------------

[Reversible computing] is a special type of computation where any steps taken can be trivially reversed. This paradigm helps to reduce the amount of code by half in some cases, and to perform inverse actions "for free", such as when compressing/uncompressing, encrypting/unencrypting, compiling/uncompiling, etc.

Reversible computers can still do all the same things irreversible computers can, but they tend to produce extraneous data called "garbage". Garbage is a by-product from computing a reducive operation, but by itself isn't relevant to the programmer's goals. Usually it gets thrown away, but it can be useful to keep around when *undoing* a computation, such as decompiling or decompressing.

Rever only has reversible operations and encourages programmers to do any garbage handling themselves. Please keep in mind that although Rever makes every attempt to keep garbage to a minimum, it cannot guarantee that the code you write will not be garbage. In order to reduce any uncertainty regarding this, we recommend assuming that any and all code is garbage.


What's Rever like?
------------------

Check out [SYNTAX.md](./SYNTAX.md) and the example programs in `examples/` to see more.


Thank you
---------

Like many of my personal projects, Rever is a labor of love. If you like it or would like to see it grow, you can make a donation at [my Ko-fi] account.

[copy-in copy-out]: https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_copy-restore
[injective]: https://en.wikipedia.org/wiki/Injective_function
[Reversible computing]: https://en.wikipedia.org/wiki/Reversible_computing
[my Ko-fi]: https://ko-fi.com/tenelevenx
