Rever Syntax
============

This guide starts from small concepts and then builds up. If you're more of a top-down person, I recommend reading this guide in reverse.


Literals
--------

In addition to the usual support for decimal (`321`), binary (`0b1011`), and hexadecimal (`0x10c`) literals, Rever also supports [bijective numerals].

These kinds of numbers always start with the digit `0`, no matter their value. The digits after it can be `1` through `9` or `A`/`a`, which represents a value of 10 at that position. Separators can be inserted between any digits with `'`.

This means `019` is 19, `01A` is 20, `021` is 21, and so on. `020` is not a valid number. `09'99A` is 10,000. `0A'AAA` is 11,110. Zero is just `0`. (And yes, this means every `0` in your program is secretly a bijective numeral, but don't tell anyone.)

This was included to make number parsing easier in Rever in the future in case it becomes [self-hosting], since bijective functions are trivially reversible. However, since this makes numbers a bit harder to read, they're not mandatory. You can always use decimal form instead by starting with any digit `1` through `9`.

[bijective numerals]: https://en.wikipedia.org/wiki/Bijective_numeration
[self-hosting]: https://en.wikipedia.org/wiki/Self-hosting_(compilers)


Simple statements
-----------------

The most trivial statement is `skip`. It does absolutely nothing. It may be useful for being explicit that nothing should be done in some cases.

Next step is unary operators. These are `not` and `-`, which perform a bit-wise NOT and a two's complement negation, respectively.

	not flag
	-vec

Next come the assignment statements. These include add-assign (`+=`), sub-assign (`-=`), and xor-assign (`:=`). Respectively, they each do "increment by value", "decrement by value", and "xor with value".

	hash.(0)    := 3
	sum         += 4
	name.len    -= sum + 1

There's also left-rotate (`:<`) and right-rotate (`:>`), which rotate the bits in a number by the given amount. They're very similar to bit shifts, except the last and first bits wrap around to the other end.

	n := 1  # n = 0b001
	n :< 2  # n = 0b100
	n :> 1  # n = 0b010

Lastly, there's swap (`<>`), which takes 2 variables and swaps their values.

	a := 3
	b := 5
	a <> b   # a = 5 and b = 3


Calling and Uncalling
---------------------

Where Rever really shines is with procedure calls. You can call a procedure with `do` to run it forwards, or with `undo` to run it backwards. `undo` will perform each statement in a procedure both in reverse order *and* opposite from its original action.

For example, if you have a procedure with the following statements:

```
not x
x += 1
```

Then calling it with `undo` would perform the procedure as if it were written as:

```
x -= 1
not x
```

**Note**: procedures are always called with "in-out" parameters, which means that when the procedure finishes, the final value of the parameters will be copied back to the caller.


Compound statements
-------------------

You may have heard of "variables". In Rever, a variable is declared by giving it a name and initial value, then a scope for which it will be live, and then a value to deinitialize it. Because of this structure, variables must be dropped syntactically in reverse order to how they were declared. This enforces what in type theory is called an [ordered type system](https://en.wikipedia.org/wiki/Substructural_type_system#Ordered_type_system).

```
# what someone would write
var i := 1
var j := f i + 1
i += 2
j -= 1
drop j := f (i - 2)
drop i := 3

# what the computer sees (semantically equivalent)
var i := 1
	var j := f i + 1
		i += 2
		j -= 1
	drop j := f(i - 2)
drop i := 3
```

If-else branches can have at most 4 parts: the test, the code block to run if it passes, an optional else-block, and an optional assertion. Only the assertion and test are swapped when running backwards, not the code blocks.

The value of the assertion *must* match the value of the test at the end of the branch. In other words, they *must* both be true or both false. If the assertion is not given, it's assumed to be the same as the test. Note that this is *not always* what you may want.

```
if a = 0
	b += 1
fi

# same as above
if a = 0
	b += 1
fi a = 0

# omitting assertion wouldn't work here
if a = b
	a += b
else
	a -= b
fi a = 2*b
```

As a general rule, if the variables in the condition are modified while in the branch, you *will* have to write an assertion.

You can also have continuous `else if` sections as such:

```
if a = 0
	do smth0
else if a = 1
	do smth1
else if a = 2
...
else if a = 10
	do smth10
else
	do smth
fi a = 10
...
fi a = 2
fi a = 1
fi a = 0

# this is the same as above
if a = 0
	do smth0
else
	if a = 1
		do smth1
	else
		if a = 2
		...
		else
			if a = 10
				do smth10
			else
				do smth
			fi a = 10
		...
		fi a = 2
	fi a = 1
fi a = 0
```

Loops are a bit more complicated. They have an assertion at the start, a do-block (the main loop code), a test for when to stop, and a back-block that runs if the test fails.

The initial assertion *must only* be true at the start. The do-block is then executed, followed by the test. If it's true, the loop is exited. Otherwise, the back-block is executed, the assertion is checked to be false, and we loop from there. Like if-else blocks, only the test and assertion are swapped when going in reverse.

```
~ i goes from 0 to 5 (exclusive)
from i = 0
	do print {"hello"}
	i += 1
	do println {" world!"}
until i = 5
loop

~ i goes from 0 to 5 (inclusive)
from i = 0
	do print {"hello"}
until i = 5
	i += 1
	do println {" world!"}
loop
```

The back-block gives the flexibility of running the test before actually executing code, or to have code that runs only after the test fails.

### Note about assertions

Assertions should allow a statement running in reverse to determine what value a variable should have at the end of its life, which branch to take for conditionals, or what the starting condition is in loops.

In conditionals, they should reflect what changes the first code block made to the variables. If the variables being tested aren't changed in either branch of code, then you can safely have the assertion be the same as the test. If variables in the test *are* being changed, the assertion *cannot* be the same as the test, and must reliably choose which branch should execute when going in reverse.

In loops, the assertion can be the end value of the iterating variable, or a predicate that depends on any variable whose value depends on the loop.


Procedures and Functions
------------------------

Following in Rever's "dualist" philosophy, it has both procedures and functions, and focuses on emphasizing each of their strengths.

**Procedures** focus on control-flow and mutable state. They run a series of statements, optionally taking a list of parameters. Parameters have [copy-in copy-out semantics] which means that if a procedure's parameter is not marked with `const`, then the value of the variable given by the caller could change after the call. They can also have side-effects.

**Functions** focus on data-flow and purity. They evaluate a list of immutable arguments and return a value. This means that they can be used in expressions. Unlike procedures, functions cannot have side-effects; any variables or statics outside of their scope are not accessible. They can still access constants and other functions.

Use procedures if you want:
+ side-effects,
+ mutable state,
+ a more imperative/control-flow style, or
+ more control over system details.

Use functions if you want:
+ a return value,
+ no side-effects,
+ a more functional/data-flow style, or
+ more ease of development.

### Procedures

A procedure is declared like this:

```
proc add(x: int, const c: int)
	x += c
return
```

### Functions

A function can be declared in the following ways:

```
# one-line form
fn succ(x): int = x + 1

# multiline form
fn ackermann(m: int, n: int): int
	if m = 0
		n + 1
	else if n = 0
		ackermann(m - 1, 1)
	else
		ackermann(m - 1, ackermann(m, n - 1))
```

[copy-in copy-out semantics]: https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_copy-restore