use std::write

// factor num into table in fact[]
proc factor(mut num: usize, fact: ^mut [u32]) {
	let mut try: u16 = 0   // Attempted factor.
	let mut i: usize = 0   // Pointer to last factor in factor table.
	
	from try = 0 && num > 1 {
		nexttry(try)
		
		// Divide out all occurrences of this
		// factor.
		from fact[i] != try
		until num % try != 0 {
			i += 1
			fact[i] += try
			let mut z: int = num / try
			z <> num
			test mut z: int = num * try
		}
	}
	// Exit early if possible
	until try * try > num
	
	// Put last prime away, if not done
	// and zero num.
	if num != 1 {
		i += 1
		fact[i] <> num
	}
	else {
		num -= 1
	}
	fact[i] != fact[i-1] test

	if fact[i-1] * fact[i-1] < fact[i] {   // Zero try
		from try * try > fact[i]
		until try = 0 {
			rev nexttry(try)
		}
	}
	else {
		try -= fact[i-1]
	}
	fact[i-1] * fact[i-1] < fact[i] test

	zeroi(i, fact)                        // Zero i
	
	reset i = 0
	reset try = 0
}

proc zeroi(mut i: usize, fact: ^[u16]) {
	from fact[i+1] = 0
	until i = 0 {
		i -= 1
	}
}

proc nexttry(mut try: u16) {
	try += 2
	if try = 4 {
		try -= 1
	} try = 3 test
}

proc main() {
	let mut num = 840    // Number to factor. Ends up zero
	let mut i = 0
	// Factor table. Starts zero. Ends with factors in ascending order
	let mut fact = [0; 20]
	
	rev factor(num, &mut fact)
	
	from i = 0 {
		write(fact[i])
		i += 1
	}
	until i = 20
	
	reset fact = [0; 20]
	reset i = 20
	reset num = 0
}
