Rever Syntax
============

## Simple statements

The most trivial statement is `skip`. It does absolutely nothing. Maybe you'll want it for being explicit that nothing should be done in some cases.

Next come the assignment statements. These include add-assign (`+=`), sub-assign (`-=`), and xor-assign (`:=`). They each do what their names say.

	hash.(0)    := 3
	sum         += 4
	name.len    -= sum + 1

There's also left-rotate (`:<`) and right-rotate (`:>`), which rotate the bits in a number by the given amount. They're very similar to bit shifts, except the first and last bits wrap around to the other end.

	n := 1  ~ n = 0b001
	n :< 2  ~ n = 0b100
	n :> 1  ~ n = 0b010

Lastly, there's swap (`<>`), which takes 2 variables and swaps their values.

	a := 3
	b := 5
	a <> b   ~ a = 5 and b = 3

Now we get into the interesting stuff: procedure calls. You can either call a procedure with `do` to run it forwards, or with `undo` to run it backwards. `undo` will recursively reverse and invert all statements in a procedure before the call. They both have 3 forms that you can use depending on the number of parameters the procedure has or whether you prefer a multiline call.

```
do subtask
do print: "hello world!"
do something(
	f(x) + 3,
	name
)
```

(Note: procedures are always called with "in-out" parameters, which means that when the procedure finishes, the final value of the parameters will be copied back to the caller.)

## Compound statements

You may have heard of "variables". In Rever, a variable is declared by giving it a name and initial value, then a scope for which it's "live", and then a value to deinitialize it. Because of this structure, variables must be dropped in reverse order to how they were declared.

```
var i := 1
	var j := f(i) + 1
		i += 2
		j -= 1
	drop j := f(i - 2)
drop i := 3

~ same as above
var i := 1
var j := f(i) + 1
i += 2
j -= 1
drop j := f(i - 2)
drop i := 3
```

If-else branches can have at most 4 parts: the test, the code block to run if it passes, an optional else-block, and an optional assertion. Only the assertion and test are swapped when running backwards. The value of the assertion *must* match the value of the test at the end of the branch. If the assertion is not given, it's assumed to be the same as the test. Note that this is *not always* what you may want.

```
if a = 0
	b += 1
fi

~ same as above
if a = 0
	b += 1
fi a = 0

~ omitting assertion wouldn't work here
if a = b
	a += b
else
	a -= b
fi a = 2*b
```

If the variables in the condition could change while in the branch, you *will* have to write an assertion.

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
fi ...
fi a = 2
fi a = 1
fi a = 0

~ this is the same as above
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

Loops are a bit more complicated. They have an assertion at the beginning, a do-block or a back-block (or both), and a test. The initial assertion can only be true at the start, and *must* be false while looping. The do-block is then executed. After that, the test is evaluated to see if the loop should stop. If the test is true, we exit the loop. Otherwise, the back-block is executed, the assertion is checked to be false, and it loops back again to running the do-block. Like conditionals, only the test and assertion are swapped when going in reverse.

```
~ i goes from 0 to 5 (exclusive)
from i = 0
	do println: "hi"
	i += 1
until i = 5
loop

~ i goes from 0 to 5 (inclusive)
from i = 0
	do println: "hi"
until i = 5
	i += 1
loop
```

The back-block gives the flexibility of running the test before actually executing code, or to have code that runs only after the test fails.

### Note about assertions

Assertions should allow a statement running in reverse to determine what value a variable should have at the end of its life, which branch to take for conditionals, or what the starting condition is in loops.

In conditionals, they should reflect what changes the first code block made to the variables. If the variables being tested aren't changed in either branch of code, then you can safely have the assertion be the same as the test. If variables in the test *are* being changed, the assertion *cannot* be the same as the test, and must reliably choose which branch should execute when going in reverse.

In loops, the assertion can be the end value of the iterating variable, or a predicate that depends on any variable whose value depends on the loop.


Procedures and Functions
------------------------

Following in Rever's "dualist" philosophy, it has both procedures and functions.

**Procedures** run a series of statements, optionally taking a list of arguments. Parameters have [copy-in copy-out] semantics which means that if a procedure marks one of its parameters as mutable with `var`, then the value of the variable given by the caller could change after the call. They can also have side-effects.

**Functions** evaluate a list of immutable arguments and return a value. This means that they can be used in expressions. Unlike procedures, functions cannot have side-effects; any variables or statics outside of their scope are not accessible. They can still access constants, enum variants, and other functions.

Use procedures if you want:
+ side-effects,
+ mutable state,
+ a more imperative/control-flow style,
+ or more control over memory use.

Use functions if you want:
+ a return value,
+ no side-effects,
+ a more functional/data-flow style,
+ or more ease of development.

[copy-in copy-out]: https://en.wikipedia.org/wiki/Evaluation_strategy#Call_by_copy-restore


Features under construction
---------------------------

It can become a bit tedious (and error-prone!) to repeat expressions multiple times in different places. That's why some alternate control structures are being considered for some special-case code.

### Chained comparisons

*A la* Python:

```
if start <= x < end
    do something
fi
```

### Special boolean operators

In the same vein as chained comparisons, instead of using `&&` for AND and `||` for OR as short-circuiting boolean operators, we can have special syntax in the conditional statement.

```
if a < x < b, x != 0; x = -1
    do something
fi
```

This would be the same as `a < x && x < b && x != 0 || x = -1` in C-like languages. `and` and `or` can be used when short-circuiting is desired, while the special syntax can behave like in Pascal.

### `match` blocks

A pattern-matching statement like `match` would be useful when a variable is being checked for multiple values.

Instead of

```
if a = 0
	do something1
else if a = 1
	do something2
else if a = 2
	~ ...
fi
fi
fi
```

you can write something like this (syntax subject to change):

```
match a
	0: do something1
	1: do something2
	...
	_: skip
end
```


### `for` loops

A common case for loops is to iterate through a range of numbers, a list of items, etc. So a `for` loop is being considered:

```
for i in 0..100   ~ for numbers
    do something
loop

for i in list     ~ for finite lists
    do something
loop
```

### Inline variable declaration

It can become very tedious to always be the one to initialize variables, only to have it passed to a procedure. Consider the following:

```
proc print_file(path)
	var bytes := 80
	var file := nil
	var buf := nil
	
	do load: path, file
	do take: file, buf, bytes
	do print: buf, bytes
	undo load: path, file
	
	drop buf := nil
	drop file := nil
	drop bytes := 80
end
```

This could be better written much more succinctly like this:

```
proc print_file(path)
	var bytes := 80
	
	do load: path, var file
	do take: file, var buf, bytes
	do print: drop buf, bytes
	undo load: path, drop file
	
	drop bytes := 80
end
```

More specifically, the parameter marked with `var` will take the value that the procedure being called expects. For example, if a procedure starts with `from i = 0` and `i` is a `var` parameter, then `i` will be initialized with the value of 0.

