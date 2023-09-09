Features under consideration
============================

It can become a bit tedious (and error-prone!) to repeat expressions multiple times in different places. That's why some alternate control structures are being considered for some special-case code.


Chained comparisons
-------------------

*A la* Python:

```
if start <= x < end
    do something
fi
```


Special boolean operators
-------------------------

In the same vein as chained comparisons, instead of using `&&` for AND and `||` for OR as short-circuiting boolean operators, we can have special syntax in the conditional statement.

```
if a < x < b, x != 0; x = -1
    do something
fi
```

This would be the same as `a < x && x < b && x != 0 || x = -1` in C-like languages. `and` and `or` can be used when short-circuiting is desired, while the special syntax can behave like in Pascal.


`match` blocks
--------------

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
	0 -> do something1
	1 -> do something2
	...
	_ -> skip
end
```


`for` loops
-----------

A common case for loops is to iterate through a range of numbers, a list of items, etc. So a `for` loop is being considered:

```
for i in 0..100   ~ for numbers
    do something
loop

for i in list     ~ for finite lists
    do something
loop
```


Inline variable declaration
---------------------------

It can become very tedious to always be the one to initialize variables, only to have it passed to a procedure. Consider the following:

```
proc print_file(path)
	var bytes := 80
	var fd := nil
	var buf := nil
	
	do load: path, fd
	do take: fd, buf, bytes
	do print: buf, bytes
	undo load: path, fd
	
	drop buf := nil
	drop fd := nil
	drop bytes := 80
end
```

This could be written much more succinctly like this:

```
proc print_file(path)
	var bytes := 80
	
	do load: path, var fd
	do take: fd, var buf, bytes
	do print: drop buf, bytes
	undo load: path, drop fd
	
	drop bytes := 80
end
```

More specifically, the parameter marked with `var` will take the value that the procedure being called expects. For example, if a procedure starts with `from i = 0` and `i` is a `var` parameter, then `i` will be initialized with the value of 0.


Singleton syntax
----------------

Sometimes you want to specify a value without having a separate type declaration. This could be achieved by inlining the type information with the values that you'd like the variable to have.

```
struct Person
	age: u8
	name: str
end

let bob = Person
	age = 18
	name = "bob"
end

# can be replaced with

let bob = struct
	age: u8 = 18
	name: str = "bob"
end
```
