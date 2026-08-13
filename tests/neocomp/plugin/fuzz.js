import { writeFileSync, unlinkSync } from 'node:fs';
import { resolve } from 'node:path';
import { neocomp } from '@neocomp/vite-plugin'; // <-- Update this path to your plugin file!

// Mock dependencies required by the compiled output so we can evaluate it
const mockEnv = `
	const html = { __add: () => {} };
	const build = { html: { __add: () => {} } };
	const myVar = 42;
`;

// Helper: Pick a random item from an array
const pick = (arr) => arr[Math.floor(Math.random() * arr.length)];

// Generate random JavaScript expressions and nested templates
function generateFuzzJS (depth = 0) {
	if (depth === 0) {
		// Base cases: Strings with tricky characters that usually break parsers
		return pick([
			`"just a string"`,
			`'string with { and }'`,
			`"string with \\" and \\' and \\\` "`,
			`123`,
			`myVar`,
			`"{}"`,
		]);
	}

	const choices = [
		// 1. Math / logical expression
		() => `(${generateFuzzJS(depth - 1)} + ${generateFuzzJS(depth - 1)})`,

		// 2. Standard template literal
		() => `\`standard \\\` temp \${${generateFuzzJS(depth - 1)}} end\``,

		// 3. HTML Tagged Template (what the plugin actually compiles)
		() => `html\`<div class="test">\${${generateFuzzJS(depth - 1)}}</div>\``,

		// 4. HTML Template with escaped dollar signs (should NOT trigger interpolation)
		() => `html\`<span> \\\${\${${generateFuzzJS(depth - 1)}}} </span>\``,

		// 5. Build prefix
		() => `build.html\`\${${generateFuzzJS(depth - 1)}}\``,

		// 6. Template with no expressions
		() => `html\`<div>static content \\\` </div>\``,
	];

	return pick(choices)();
}

async function runFuzzer (iterations = 1000, maxDepth = 4) {
	console.log(`🚀 Starting Fuzzer (${iterations} iterations)...`);
	let passed = 0;

	// Initialize your plugin
	const plugin = neocomp({ include: [resolve('./fuzz-temp/')] });

	for (let i = 0; i < iterations; i++) {
		// 1. Generate a complex nested JS string
		const rawCode = `${mockEnv}\nconst result = ${generateFuzzJS(maxDepth)};`;

		// 2. Write it to a temporary file so the Vite plugin can load it
		const tempFile = resolve(`./fuzz-temp/fuzz_${i}.ts`);
		writeFileSync(tempFile, rawCode, { encoding: 'utf-8' });

		try {
			// 3. Run the Vite Plugin load hook
			const result = await plugin.load.call({}, tempFile);

			if (result && result.code) {
				try {
					// 4. SYNTAX CHECK: Does the output compile as valid JS?
					// If the plugin mangled quotes or brackets, this will throw a SyntaxError.
					new Function(result.code);
					passed++;
				} catch (syntaxErr) {
					console.error(`\n❌ Fuzz Iteration ${i} Failed: Invalid JavaScript Output!`);
					console.error(`--- ORIGINAL CODE ---`);
					console.error(rawCode);
					console.error(`--- COMPILED CODE ---`);
					console.error(result.code);
					console.error(`--- ERROR ---`);
					console.error(syntaxErr.message);
					unlinkSync(tempFile);
					process.exit(1);
				}
			}
		} catch (pluginErr) {
			console.error(`\n❌ Fuzz Iteration ${i} Failed: Plugin Crashed!`);
			console.error(`--- ORIGINAL CODE ---`);
			console.error(rawCode);
			console.error(`--- ERROR ---`);
			console.error(pluginErr.message);
			unlinkSync(tempFile);
			process.exit(1);
		}

		// Cleanup temp file
		unlinkSync(tempFile);
	}

	console.log(`✅ Fuzzer finished! ${passed}/${iterations} passed successfully.`);
}

// Ensure temp dir exists
import { existsSync, mkdirSync } from 'node:fs';
if (!existsSync('./fuzz-temp')) mkdirSync('./fuzz-temp');

// Run the fuzzer
runFuzzer(1000, 4);
