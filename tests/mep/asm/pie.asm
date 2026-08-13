// ============================================================
// Fast Pi Calculator for MEP
//
// Uses Machin's formula:
//   pi = 16*arctan(1/5) - 4*arctan(1/239)
//
// arctan(1/x) is computed as a fixed-point Taylor series:
//   arctan(1/x) = 1/x - 1/(3x^3) + 1/(5x^5) - 1/(7x^7) + ...
//
// All values are scaled by S = 10^15 and kept as 64-bit
// integers, so "pi_fixed" (stored in `result`) equals
// round(pi * 10^15).
//
// This converges fast: each new term is divided by x^2, so
// arctan(1/5) needs ~11 terms and arctan(1/239) needs ~2
// terms to exhaust 10^15 of precision.
// ============================================================

start:
    ld  r1, [scale]        // r1 = S = 10^15

    // --- arctan(1/5) * S ---
    mov r10, 5              // x = 5
    br  r20, arctan          // call arctan(x) -> result in r11
    mov r21, r11             // r21 = S*arctan(1/5)

    // --- arctan(1/239) * S ---
    mov r10, 239             // x = 239
    br  r20, arctan
    mov r22, r11             // r22 = S*arctan(1/239)

    // --- pi = 16*r21 - 4*r22 ---
    mov r2, 16
    mult r23, r21, r2         // r23 = 16 * S*arctan(1/5)
    mov r2, 4
    mult r24, r22, r2         // r24 = 4 * S*arctan(1/239)
    sub  r25, r23, r24        // r25 = S * pi

    // store result and stop
    add r30, pc, result       // r30 = &result
    st  r25, [r30]
    halt

// ------------------------------------------------------------
// arctan(1/x) * S, fixed point.
// in:  r10 = x, r1 = S
// out: r11 = S * arctan(1/x)
// clobbers: r12-r16
// return address expected in r20 (set by caller via `br link, arctan`)
// ------------------------------------------------------------
arctan:
    div  r12, r1, r10         // term = S / x            (k=1 term)
    mov  r11, r12              // sum  = term
    mult r13, r10, r10          // x2 = x * x
    mov  r14, 1                 // k = 1
    mov  r16, 1                 // sign toggle: 1 = next term subtracts

arctan_loop:
    div  r12, r12, r13         // term /= x^2
    comp.eq c0, r12, r0
    br.true c0, arctan_done     // term underflowed to 0 -> converged

    add  r14, r14, 2            // k += 2
    div  r15, r12, r14          // delta = term / k
    comp.eq c0, r15, r0
    br.true c0, arctan_done     // delta underflowed to 0 -> converged

    comp.eq c0, r16, r0
    br.true c0, arctan_add

    sub  r11, r11, r15          // sum -= delta
    mov  r16, 0                  // next term adds
    br   arctan_cont

arctan_add:
    add  r11, r11, r15          // sum += delta
    mov  r16, 1                  // next term subtracts

arctan_cont:
    br arctan_loop

arctan_done:
    jmp r20                     // return to caller

// ------------------------------------------------------------
// Data
// ------------------------------------------------------------
scale:  u64 1000000000000000    // 10^15 fixed-point scale
result: u64 0                    // holds S * pi after `start` runs