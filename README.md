Rever
=====

Rever is an experimental programming language trying out different ideas at the same time. These ideas include:
* reversible computation
* [predictive parser]
* significant newlines
* has both procedures and functions
* [copy-in copy-out] semantics
* synchronizing AST (soon)

Its syntax is inspired by [Janus], Rust, Pascal, and Haskell. It's in the same syntax family as Crunch and similar languages.

[Janus]: https://en.wikipedia.org/wiki/Janus_(time-reversible_computing_programming_language)
[predictive parser]: https://en.wikipedia.org/wiki/Predictive_parser
<!-- [Bob](https://link.springer.com/chapter/10.1007/978-3-642-29517-1_3). -->


Installation
------------

First, [install Rust], then download the ZIP file for this project. Once you've unzipped it and gone into the directory, you can run:

    cargo install --path .

This will install the program and allow you to interpret Rever files using `rever <file>`.

If you're afraid of commitment like I am (it's an experimental language, after all!), you can use this command to invoke the interpreter without installing anything:

	cargo run -- path/to/file

As of now, the interpreter can only do math and print to the terminal. Try some of the examples in the `examples/` folder!

[install Rust]: https://www.rust-lang.org/tools/install


What is reversible computing?
-----------------------------

[Reversible computing] is a special type of computation where any steps taken can be trivially reversed. This paradigm helps to reduce the amount of code by half in some cases, and to perform inverse actions "for free", such as when compressing/uncompressing, encrypting/unencrypting, compiling/uncompiling, etc.

Reversible computers can still do all the same things irreversible computers can, but they tend to produce extraneous data called "garbage". Garbage is a by-product from computing a reducive operation, but which isn't relevant to the programmer's goals.

Usually when only going in one direction, people (and irreversible computers) don't care for it and throw it away. However, this "garbage" can be very useful when going in the opposite direction, such as when reverse-engineering or uncompiling some assembly. This is why Rever only has reversible operations and encourages programmers to do any garbage handling themselves.

Please keep in mind that although Rever makes every attempt to keep garbage to a minimum, it cannot guarantee that the code you write will not be garbage. In order to reduce any uncertainty regarding this, we recommend assuming that any and all code is garbage.

To get a better grasp of how a reversible language works, try the [Janus playground]!


What's Rever like?
------------------

If you've already tried Janus with the above link, then you'll notice that Rever's syntax is very similar to that of Janus. It mostly relies on newlines and keywords (e.g. `end`, `fi`, `loop`) to denote blocks, rather than braces or indentation like in other languages.

At the top level of a Rever file there are *items*, which include procedures, functions, types, modules, etc. They are much like those in Rust.

To learn more, check out [SYNTAX.md] and the example programs in `examples/`.

[SYNTAX.md]: ./SYNTAX.md


Thank you
---------

Like many of my personal projects, Rever is a labor of love. If you like it or would like to see it grow, consider becoming a monthly contributor on [my Patreon] page, or make a one-time donation at [my Ko-fi] account.

[copy-in copy-out]: https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_copy-restore
[injective]: https://en.wikipedia.org/wiki/Injective_function
[Reversible computing]: https://en.wikipedia.org/wiki/Reversible_computing
[Janus playground]: http://topps.diku.dk/pirc/janus-playground/
[my Patreon]: https://www.patreon.com/1011X
[my Ko-fi]: https://ko-fi.com/tenelevenx
