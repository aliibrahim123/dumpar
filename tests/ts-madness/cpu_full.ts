import type { run } from "../cpu/index.ts";
import * as asm from "../cpu/asm.ts";

type program = run<[[
	asm.mov_imd<1, [1, 0, 0, 0]>,
	asm.mov_imd<2, [1, 0, 0, 0]>,
	asm.add<3, 1, 2>,
	asm.halt
]], {}>['regs'][3]; // => 3