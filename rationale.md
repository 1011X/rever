# Language Rationale

## Semicolons

### Why Rust Has Them

Semicolons do 2 things:
* Signify the end of a statement.
* Signify to return unit (`()`) type when present.

### Should the language have them?

Since procedures in reversible languages never return anything, they shouldn't
be used for that purpose.

We could have them for ending statements. There likely won't be keyword-based
statement separation (like Janus) in the future, so I'll stick with the safe
option of having them.

## Calling functions

Because all programming languages call in a forward manner, there's not really
anything to base a decision on how to *un*call a procedure besides how Janus
does it.

I was thinking of using something like `(x, y)f` as the uncall equivalent of
`f(x, y)`. I think that could work... No counter-examples or ambiguities come to
mind. It would also reduce some of the clutter of using `call` & `uncall`, and
kinda parallels the `if x {...} y assert` syntax.


