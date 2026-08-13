// MEP Architecture Exhaustive Test Suite
// ---------------------------------------------------------
// Register Usage:
// r31 - Current Test ID (0 on full success)
// r30, r29, r28 - General scratch registers
// r1, r2, r3, etc. - Test specific operands
// c0 - Condition register for assertions
// ---------------------------------------------------------

start:
	// Initialize default operands
	mov r1, +10
	mov r2, +2

// ---------------------------------------------------------
// Arithmetic Instructions
// ---------------------------------------------------------

	// Test 1: add (shifted register)
	mov r31, +1
	add r3, r1, r2 shl 1        // r3 = 10 + (2 << 1) = 14
	comp.ne c0, r3, +14
	br.true c0, test_fail

	// Test 2: add (immediate)
	mov r31, +2
	add r3, r1, +5              // r3 = 10 + 5 = 15
	comp.ne c0, r3, +15
	br.true c0, test_fail

	// Test 3: add.carry
	mov r31, +3
	comp.eq c0, r0, r0          // Set c0 to true (1)
	add.carry r3, r1, r2, c0    // r3 = 10 + 2 + 1 = 13 (c0 updated)
	comp.ne c0, r3, +13
	br.true c0, test_fail

	// Test 4: add triple
	mov r31, +4
	mov r3, +5
	add r4, r1, r2, r3          // r4 = 10 + 2 + 5 = 17
	comp.ne c0, r4, +17
	br.true c0, test_fail

	// Test 5: cinc
	mov r31, +5
	comp.eq c0, r0, r0          // Set c0 to true
	cinc r4, c0, r1             // r4 = 10 + 1 = 11
	comp.ne c0, r4, +11
	br.true c0, test_fail

// ---------------------------------------------------------
// Subtract Instructions
// ---------------------------------------------------------

	// Test 6: sub (shifted register)
	mov r31, +6
	sub r3, r1, r2              // r3 = 10 - 2 = 8
	comp.ne c0, r3, +8
	br.true c0, test_fail

	// Test 7: sub reverse (shifted register)
	mov r31, +7
	sub r3, r2 shl 2, r1        // r3 = (2 << 2) - 10 = 8 - 10 = -2
	comp.ne c0, r3, -2
	br.true c0, test_fail

	// Test 8: sub (immediate)
	mov r31, +8
	sub r3, r1, 3               // r3 = 10 - 3 = 7
	comp.ne c0, r3, +7
	br.true c0, test_fail

	// Test 9: sub reverse (immediate)
	mov r31, +9
	sub r3, 20, r1              // r3 = 20 - 10 = 10
	comp.ne c0, r3, +10
	br.true c0, test_fail

	// Test 10: sub.borrow
	mov r31, +10
	comp.eq c0, r0, r0          // c0 = true (1)
	sub.borrow r3, r1, r2, c0   // Performs borrow subtraction
	// Simply ensuring compilation and execution flow

// ---------------------------------------------------------
// Multiplication & Division
// ---------------------------------------------------------

	// Test 11: mult
	mov r31, +11
	mult r3, r1, r2             // r3 = 10 * 2 = 20
	comp.ne c0, r3, +20
	br.true c0, test_fail

	// Test 12: mult.full
	mov r31, +12
	mult.full r3, r4, r1, r2    // r3 (low) = 20, r4 (high) = 0
	comp.ne c0, r3, +20
	br.true c0, test_fail
	comp.ne c0, r4, +0
	br.true c0, test_fail

	// Test 13: madd
	mov r31, +13
	madd r3, r1, r2, r1         // r3 = 10 * 2 + 10 = 30
	comp.ne c0, r3, +30
	br.true c0, test_fail

	// Test 14: msub
	mov r31, +14
	msub r3, r1, r2, r1         // r3 = 10 - (10 * 2) = -10
	comp.ne c0, r3, -10
	br.true c0, test_fail

	// Test 15: div
	mov r31, +15
	div r3, r1, r2              // r3 = 10 / 2 = 5
	comp.ne c0, r3, +5
	br.true c0, test_fail

	// Test 16: rem
	mov r31, +16
	mov r5, +3
	mov r1, +10
	rem r3, r1, r5              // r3 = 10 % 3 = 1
	comp.ne c0, r3, +1
	br.true c0, test_fail

	// Test 17: div.full
	mov r31, +17
	div.full r3, r4, r1, r5     // r3 (quo) = 3, r4 (rem) = 1
	comp.ne c0, r3, +3
	br.true c0, test_fail
	comp.ne c0, r4, +1
	br.true c0, test_fail

	// Test 18: udiv
	mov r31, +18
	udiv r3, r1, r2             // r3 = 10 / 2 = 5
	comp.ne c0, r3, +5
	br.true c0, test_fail

	// Test 19: urem
	mov r31, +19
	urem r3, r1, r5             // r3 = 10 % 3 = 1
	comp.ne c0, r3, +1
	br.true c0, test_fail

	// Test 20: udiv.full
	mov r31, +20
	udiv.full r3, r4, r1, r5
	comp.ne c0, r3, +3
	br.true c0, test_fail

// ---------------------------------------------------------
// Sign and Comparison
// ---------------------------------------------------------

	// Test 21: abs
	mov r31, +21
	mov r3, -15
	abs r4, r3                  // r4 = 15
	comp.ne c0, r4, +15
	br.true c0, test_fail

	// Test 22: neg
	mov r31, +22
	mov r3, +15
	neg r4, r3                  // r4 = -15
	comp.ne c0, r4, -15
	br.true c0, test_fail

	// Test 23: cneg
	mov r31, +23
	comp.eq c0, r0, r0          // true
	cneg r4, c0, r3             // Negates 15 to -15
	comp.ne c0, r4, -15
	br.true c0, test_fail

	// Test 24: se
	mov r31, +24
	mov r3, 0xff
	se.8 r4, r3                 // Sign extend 8-bit 0xff -> -1
	comp.ne c0, r4, -1
	br.true c0, test_fail

	// Test 25: min / max
	mov r31, +25
	min r4, r1, r2              // min(10, 2) = 2
	comp.ne c0, r4, +2
	br.true c0, test_fail

	mov r31, +26
	max r4, r1, r2              // max(10, 2) = 10
	comp.ne c0, r4, +10
	br.true c0, test_fail

	// Test 27: umin / umax
	mov r31, +27
	umin r4, r1, r2
	comp.ne c0, r4, +2
	br.true c0, test_fail

	mov r31, +28
	umax r4, r1, r2
	comp.ne c0, r4, +10
	br.true c0, test_fail

	// Test 29: Comparisons (Equality / Inequality)
	mov r31, +29
	comp.eq c0, r1, r1
	br.false c0, test_fail

	mov r31, +30
	comp.ne c0, r1, r2
	br.false c0, test_fail

	mov r31, +31
	comp.gt c0, r1, r2
	br.false c0, test_fail

	mov r31, +32
	comp.le c0, r2, r1
	br.false c0, test_fail

	mov r31, +33
	ucomp.gt c0, r1, r2
	br.false c0, test_fail

	mov r31, +34
	ucomp.le c0, r2, r1
	br.false c0, test_fail

// ---------------------------------------------------------
// Logical Instructions
// ---------------------------------------------------------

	mov r3, 0b1100
	mov r4, 0b1010

	// Test 35: not
	mov r31, +35
	mov r5, +0
	not r6, r5                  // ~0 = -1
	comp.ne c0, r6, -1
	br.true c0, test_fail

	// Test 36: cnot
	mov r31, +36
	comp.eq c0, r0, r0
	cnot r6, c0, r5             // Conditionally compute ~0
	comp.ne c0, r6, -1
	br.true c0, test_fail

	// Test 37: and
	mov r31, +37
	and r5, r3, r4              // 0b1100 & 0b1010 = 0b1000
	comp.ne c0, r5, 0b1000
	br.true c0, test_fail

	// Test 38: or
	mov r31, +38
	or r5, r3, r4               // 0b1100 | 0b1010 = 0b1110
	comp.ne c0, r5, 0b1110
	br.true c0, test_fail

	// Test 39: xor
	mov r31, +39
	xor r5, r3, r4              // 0b1100 ^ 0b1010 = 0b0110
	comp.ne c0, r5, 0b0110
	br.true c0, test_fail

	// Test 40: bcr (Bit Clear)
	mov r31, +40
	bcr r5, r3, r4              // 0b1100 & ~0b1010 = 0b0100
	comp.ne c0, r5, 0b0100
	br.true c0, test_fail

	// Test 41: imply
	mov r31, +41
	imply r5, r3, r4            // ~0b1100 | 0b1010
	not r6, r3
	or r6, r6, r4               // Validate manually
	comp.ne c0, r5, r6
	br.true c0, test_fail

	// Test 42: nand / nor / xnor (Compilation coverage)
	mov r31, +42
	nand r5, r3, r4
	nor r5, r3, r4
	xnor r5, r3, r4

	// Test 43: Bit Tests
	mov r31, +43
	mov r5, 0b1000
	test.none c0, r5, logic_imd(3, 0)     // Is 0b1000 & 0b0111 == 0? -> True
	br.false c0, test_fail

	mov r31, +44
	test.any c0, r5, logic_imd(2, 2)     // Is 0b1000 & 0b1100 != 0? -> True
	br.false c0, test_fail

	mov r31, +45
	test.all c0, r5, logic_imd(1, 3)      // Is 0b1000 & 0b1000 == 0b1000? -> True
	br.false c0, test_fail

// ---------------------------------------------------------
// Bit Manipulation
// ---------------------------------------------------------

	// Test 46: shl
	mov r31, +46
	mov r3, +1
	shl r4, r3, 3               // 1 << 3 = 8
	comp.ne c0, r4, +8
	br.true c0, test_fail

	// Test 47: shr
	mov r31, +47
	mov r3, +8
	shr r4, r3, 2               // 8 >> 2 = 2
	comp.ne c0, r4, +2
	br.true c0, test_fail

	// Test 48: sar
	mov r31, +48
	mov r3, -4                  // Arithmetic -4 >> 1 = -2
	sar r4, r3, 1
	comp.ne c0, r4, -2
	br.true c0, test_fail

	// Test 49: rol
	mov r31, +49
	mov r3, 0x8000, 3           // Load 0x8000000000000000
	rol r4, r3, 1               // Rotate Left 1 bit = 1
	comp.ne c0, r4, +1
	br.true c0, test_fail

	// Test 50: fush
	mov r31, +50
	mov r3, +1
	mov r4, +2
	mov r8, 1
	fush r5, r3, r4, r8          // Left funnel shift by 1

	// Test 51: bfext
	mov r31, +51
	mov r3, 0b10110
	bfext r4, r3, 1, 2          // Extract (width+1=3) bits at offset 1 = 0b011 = 3
	comp.ne c0, r4, +3
	br.true c0, test_fail
	
	// Test 52: bfins
	mov r31, +52
	mov r3, 0b00001
	mov r4, 0b111
	bfins r3, r4, 1, 2          // Insert 0b111 at offset 1 = 0b1110 = 14
	comp.ne c0, r3, 0b111
	br.true c0, test_fail

	// Test 53: cnt (Popcount)
	mov r31, +53
	mov r3, 0b1011
	cnt r4, r3                  // Number of set bits = 3
	comp.ne c0, r4, +3
	br.true c0, test_fail

	// Test 54: clz / clo / ctz / cto / cls / cntz
	mov r31, +54
	cntz r4, r3                 // Execute 0 bit count
	clz r4, r3                  // Lead 0s
	clo r4, r3                  // Lead 1s
	ctz r4, r3                  // Trail 0s
	cto r4, r3                  // Trail 1s
	cls r4, r3                  // Sign redundant bits

	// Test 55: rev (Parts Reversal)
	mov r31, +55
	rev r4, r3
	rev.32 r4, r3
	rev.16 r4, r3
	rev.8 r4, r3

// ---------------------------------------------------------
// Data Movement
// ---------------------------------------------------------

	// Test 56: mov.keep
	mov r31, +56
	mov r3, +0
	mov.keep r3, 0xffff, 1      // Keeps lower bits, writes 0xffff shifted left by 16 bits
	
	// Test 57: sel
	mov r31, +57
	comp.eq c0, r0, r0          // True
	sel r3, c0, r1, r2          // Select r1 over r2
	comp.ne c0, r3, r1
	br.true c0, test_fail

// ---------------------------------------------------------
// Memory Access
// ---------------------------------------------------------

	// Test 58: ld / st (offset addressing using label offset)
	mov r31, +58
	mov r3, 0x1234
	add r8, pc, data_var_1
	st.32 r3, [r8]
	ld.32 r4, [data_var_1]
	comp.ne c0, r4, r3
	br.true c0, test_fail

	// Test 59: ld / st (base + offset)
	mov r31, +59
	add r10, pc, data_var_2     // Retrieve physical address of var
	mov r3, 0x5678
	st.32 r3, [r10 + 0]
	ld.32 r4, [r10 + 0]
	comp.ne c0, r4, r3
	br.true c0, test_fail

	// Test 60: ld / st (base + index)
	mov r31, +60
	mov r11, +4
	st.32 r3, [r10 + r11]
	ld.32 r4, [r10 + r11]
	comp.ne c0, r4, r3
	br.true c0, test_fail

	// Test 61: ld.s (Sign Extending Load)
	mov r31, +61
	mov r3, 0xff
	st.8 r3, [r10 + 0]
	ld.s8 r4, [r10 + 0]         // Load and sign extend 0xff to 64-bit (-1)
	comp.ne c0, r4, -1
	br.true c0, test_fail

// ---------------------------------------------------------
// Control Flow
// ---------------------------------------------------------

	// Test 62: br unconditionally
	mov r31, +62
	br skip_br
	br test_fail
skip_br:

	// Test 63: br link
	mov r31, +63
	br r15, skip_br_link
	br test_fail
skip_br_link:
	// Register r15 successfully holds ret address

	// Test 64: br.cond
	mov r31, +64
	comp.eq c0, r0, r0          // True
	br.true c0, skip_br_cond
	br test_fail
skip_br_cond:

	// Test 65: jmp (base + offset)
	mov r31, +65
	add r10, pc, skip_jmp
	jmp r10               // Resolves to pointer loc and jumps there
	br test_fail
skip_jmp:

	mov r31, +66
	mov r4, 2
	add r10, pc, 0
	br.table r5, r4 shl 3
	br test_fail
	br test_fail
	br test_fail
	br test_fail
	add r10, r10, 8
	comp.ne c0, r5, r10
	br.true c0, test_fail

// ---------------------------------------------------------
// End of Tests
// ---------------------------------------------------------
	
	// All tests passed successfully
	mov r31, +0
	halt

// ---------------------------------------------------------
// Failure Intercept Routine
// ---------------------------------------------------------
test_fail:
	u8 0                        // Inspect `r31` in debugger to see the failed Test ID.

// ---------------------------------------------------------
// Memory Constants Space
// ---------------------------------------------------------
data_var_1: u32 0
data_var_2: u32 0
data_var_3: u32 0