import { writeFile } from "node:fs/promises";

function row (l, item, one_row = true) {
	let str = "a: {" + (one_row ? "" : "\n");
	for (let x = 0; x < l; x++) {
		if (one_row) str += `${x} => ${item(x)}, `;
		else `\t${x} => ${item(x)},\n`
	}
	str += "}";
	return str;
}
function table (l1, l2, item, ident = "") {
	let str = "a: {\n";
	for (let x = 0; x < l1; x++) {
		str += `${ident}\t${x} => b: {\n`;
		for (let y = 0; y < l2; y++) {
			str += `${ident}\t\t${y} => ${item(x, y)},\n`;
		}
		str += `${ident}\t},\n`;
	}
	str += `${ident}}`;
	return str;
}
function table3 (l1, l2, l3, item) {
	let str = "a: {\n";
	for (let x = 0; x < l1; x++) {
		str += `\t${x} => b: {\n`;
		for (let y = 0; y < l2; y++) {
			str += `\t\t${y} => c: { `;
			for (let z = 0; z < l3; z++) {
				str += `${z} => ${item(x, y, z)}, `;
			}
			str += "},\n";
		}
		str += "\t},\n";
	}
	str += "}";
	return str;
}

await writeFile("./math/tables.unim", `
// precomputed lookup tables for math operations
fn and_dg (a, b) => ${table(16, 16, (a, b) => a & b)};
fn or_dg  (a, b) => ${table(16, 16, (a, b) => a | b)};
fn xor_dg (a, b) => ${table(16, 16, (a, b) => a ^ b)};

fn add_dg (a, b, c) => ${table3(16, 16, 2, (a, b, c) => `[${(a + b + c) % 16}, ${Math.floor((a + b + c) / 16)}]`)};

// (amount, nb, filler) => funnel_shift_dg(nb, filler, amount)
fn shift_dg (am, a, b) => am: {
	0 => a,
	1 => ${table(16, 16, (a, b) => (a << 1 | b >> 3) % 16, '\t')},
	2 => ${table(16, 16, (a, b) => (a << 2 | b >> 2) % 16, '\t')},
	3 => ${table(16, 16, (a, b) => (a << 3 | b >> 1) % 16, '\t')},
};
`.trim());