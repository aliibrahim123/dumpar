import { describe, expect, it } from 'vitest';
import { parse, type Element } from '../../src/html-parser.ts'; // Adjust this import to your file path

// Helper to easily grab the first element instead of the root <html> wrapper
const getFirst = (parts: string[]) => parse(parts).children[0] as any;

describe('Template Parser Exhaustive Test Suite', () => {
	describe('1. Basic Tag & Text Parsing', () => {
		it('1. Parses empty strings', () => {
			expect(parse(['']).children).toEqual([]);
		});
		it('2. Parses pure text', () => {
			expect(parse(['Hello World']).children).toEqual(['Hello World']);
		});
		it('3. Parses a single simple tag', () => {
			expect(getFirst(['<div></div>'])).toEqual({
				tag: 'div',
				attrs: [],
				children: [],
			});
		});
		it('4. Parses sibling tags', () => {
			const ast = parse(['<div></div><span></span>']);
			expect(ast.children).toHaveLength(2);
			expect((ast.children[1] as any).tag).toBe('span');
		});
		it('5. Parses deeply nested tags', () => {
			const ast = getFirst(['<div><ul><li><b>Text</b></li></ul></div>']);
			expect(ast.children[0].tag).toBe('ul');
			expect(ast.children[0].children[0].tag).toBe('li');
			expect(ast.children[0].children[0].children[0].tag).toBe('b');
			expect(ast.children[0].children[0].children[0].children[0]).toBe('Text');
		});
		it('6. Handles tags with numbers in names', () => {
			expect(getFirst(['<h1>Title</h1>']).tag).toBe('h1');
			expect(getFirst(['<h6-custom>Title</h6-custom>']).tag).toBe('h6-custom');
		});
		it('7. Handles custom web-components', () => {
			expect(getFirst(['<my-app-root></my-app-root>']).tag).toBe('my-app-root');
		});
		it('8. Handles multiline text', () => {
			expect(getFirst(['<pre>\nline1\nline2\n</pre>']).children[0]).toBe('\nline1\nline2\n');
		});
	});

	describe('2. Attributes', () => {
		it('9. Parses unquoted attributes', () => {
			expect(getFirst(['<div id=app></div>']).attrs).toEqual([
				{
					attr: 'id',
					value: 'app',
				},
			]);
		});
		it('10. Parses single-quoted attributes', () => {
			expect(getFirst(["<div id='app'></div>"]).attrs).toEqual([
				{
					attr: 'id',
					value: 'app',
				},
			]);
		});
		it('11. Parses double-quoted attributes', () => {
			expect(getFirst(['<div id="app"></div>']).attrs).toEqual([
				{
					attr: 'id',
					value: 'app',
				},
			]);
		});
		it('12. Parses empty quoted attributes', () => {
			expect(getFirst(['<div class=""></div>']).attrs).toEqual([
				{
					attr: 'class',
					value: '',
				},
			]);
		});
		it('13. Parses boolean attributes', () => {
			expect(getFirst(['<input disabled>']).attrs).toEqual([
				{
					attr: 'disabled',
					value: '',
				},
			]);
		});
		it('14. Parses multiple mixed attributes', () => {
			const attrs = getFirst([
				'<button type="submit" disabled id=btn class=\'primary\'></button>',
			]).attrs;
			expect(attrs).toEqual([
				{ attr: 'type', value: 'submit' },
				{ attr: 'disabled', value: '' },
				{ attr: 'id', value: 'btn' },
				{ attr: 'class', value: 'primary' },
			]);
		});
		it('15. Handles special characters in attribute names', () => {
			// Framework specific bindings like Vue or Alpine
			const attrs = getFirst([
				'<div @click="do()" :class="active" x-data="{foo: 1}"></div>',
			]).attrs;
			expect(attrs[0].attr).toBe('@click');
			expect(attrs[1].attr).toBe(':class');
			expect(attrs[2].attr).toBe('x-data');
		});
		it('16. Handles liberal whitespace around "="', () => {
			expect(getFirst(['<div id \n = \n "app"></div>']).attrs).toEqual([
				{
					attr: 'id',
					value: 'app',
				},
			]);
		});
		it('17. Handles newlines inside opening tags', () => {
			expect(getFirst(['<div\nclass="test"\n></div>']).attrs[0].attr).toBe('class');
		});
	});

	describe('3. Void & Self-Closing Tags', () => {
		it('18. Auto-closes void tags (no trailing slash)', () => {
			const ast = parse(['<img src="a.png"><hr>']);
			expect(ast.children[0]).toEqual({
				tag: 'img',
				attrs: [{ attr: 'src', value: 'a.png' }],
				children: [],
			});
			expect(ast.children[1]).toEqual({
				tag: 'hr',
				attrs: [],
				children: [],
			});
		});
		it('19. Auto-closes void tags (with trailing slash)', () => {
			expect(getFirst(['<img />']).tag).toBe('img');
		});
		it('20. Self-closes normal tags via "/>"', () => {
			const ast = parse(['<div /><span />']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [],
				children: [],
			});
			expect(ast.children[1]).toEqual({
				tag: 'span',
				attrs: [],
				children: [],
			});
		});
		it('21. Handles attributes on self-closing tags', () => {
			expect(getFirst(['<Component id="1" />']).attrs).toEqual([
				{
					attr: 'id',
					value: '1',
				},
			]);
		});
		it('22. Works without spaces before "/>"', () => {
			expect(getFirst(['<div/>']).tag).toBe('div');
			expect(getFirst(['<div id="1"/>']).attrs[0].value).toBe('1');
		});
	});

	describe('4. Interpolations (Template Literal Variables)', () => {
		it('23. Interpolates text content', () => {
			const ast = parse(['<div>', '</div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [],
				children: [0],
			});
		});
		it('24. Interpolates multiple adjacent variables', () => {
			const ast = parse(['<div>', '', '</div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [],
				children: [0, 1],
			});
		});
		it('25. Mixed text and content interpolations', () => {
			const ast = parse(['<div>Hello ', ' World</div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [],
				children: ['Hello ', 0, ' World'],
			});
		});
		it('26. Interpolates attribute values', () => {
			const ast = parse(['<div id=', '></div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [{ attr: 'id', value: 0 }],
				children: [],
			});
		});
		it('27. Interpolates multiple attributes', () => {
			const ast = parse(['<div id=', ' class=', '></div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [
					{ attr: 'id', value: 0 },
					{ attr: 'class', value: 1 },
				],
				children: [],
			});
		});
		it('28. Interpolates boolean attr then interpolated attr', () => {
			const ast = parse(['<div hidden id=', '></div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [
					{ attr: 'hidden', value: '' },
					{
						attr: 'id',
						value: 0,
					},
				],
				children: [],
			});
		});
		it('29. Parses "do" element interpolations (tag-level)', () => {
			const ast = parse(['<', '>']);
			expect(ast.children[0]).toEqual({ type: 'do', arg: 0 });
		});
		it('30. Parses "do" element interpolations closing immediately', () => {
			const ast = parse(['<div><', '></div>']);
			expect(getFirst(['<div><', '></div>']).children[0]).toEqual({
				type: 'do',
				arg: 0,
			});
		});
	});

	describe('5. Comments', () => {
		it('31. Ignores standard comments', () => {
			const ast = parse(['<div><!-- comment --></div>']);
			expect(ast.children[0]).toEqual({
				tag: 'div',
				attrs: [],
				children: [],
			});
		});
		it('32. Ignores empty comments', () => {
			expect(parse(['<!---->']).children).toEqual([]);
		});
		it('33. Ignores HTML inside comments', () => {
			expect(parse(['<!-- <div id="app"></div> -->']).children).toEqual([]);
		});
		it('34. Ignores multiline comments', () => {
			expect(parse(['<!-- \nline 1\nline 2\n -->']).children).toEqual([]);
		});
		it('35. Multiple consecutive comments', () => {
			expect(parse(['<!-- a --><!-- b -->']).children).toEqual([]);
		});
	});

	describe('6. Syntax Errors & Invalid states', () => {
		it('36. Throws on unclosed tags', () => {
			expect(() => parse(['<div><span></div>'])).toThrow(
				'expected end tag "span" but got "div"',
			);
		});
		it('37. Throws on unexpected root closing tag', () => {
			expect(() => parse(['</div>'])).toThrow('unexpected end tag "div" at root');
		});
		it('38. Throws on missing ">"', () => {
			expect(() => parse(['<div'])).toThrow('unexpected end of input, expected ">"');
		});
		it('39. Throws on EOF with unclosed tags', () => {
			expect(() => parse(['<div><span>'])).toThrow('end of input with unclosed tags');
		});
		it('40. Throws on unended strings', () => {
			expect(() => parse(['<div id="app></div>'])).toThrow('unended string');
		});
		it('41. Throws on unended comments', () => {
			expect(() => parse(['<!-- oops '])).toThrow('unended comment');
		});
		it('42. Throws when attributes lack whitespace', () => {
			expect(() => parse(['<div id="1"class="2"></div>'])).toThrow('expected whitespace');
		});
		it('43. Throws on illegal characters in attribute parsing', () => {
			expect(() => parse(['<div = ></div>'])).toThrow(
				'unexpected token "=", expected "attribute name"',
			);
		});
		it('44. Throws on standalone "/"', () => {
			expect(() => parse(['</>'])).toThrow('unexpected token ">", expected "tag"');
		});
		it('45. Throws if a DO tag interpolation expects attributes', () => {
			// By design, 'in_do' expects the immediate next character to be '>'
			expect(() => parse(['<', ' id="1">'])).toThrow('unexpected token " ", expected ">"');
		});
		it('46. Throws if trying to close a void tag manually', () => {
			// Since void tags auto-pop, the </img/hr> etc actually triggers an "at root" or "mismatch" error
			expect(() => parse(['<img></img>'])).toThrow('unexpected end tag "img" at root');
		});
		it('47. Throws on nested `<` inside a tag body', () => {
			expect(() => parse(['<div <span ></span></div>'])).toThrow(
				'unexpected token "<", expected "attribute name"',
			);
		});
		it('48. Throws on random closing tag text', () => {
			expect(() => parse(['<div></div extra>'])).toThrow(
				'unexpected token "e", expected ">"',
			);
		});
		it('49. Throws if a single quote string contains double quotes without closing', () => {
			expect(() => parse(['<div id=\'test"></div>'])).toThrow('unended string');
		});
	});

	describe('7. The Very Huge Test (Test 50)', () => {
		it('50. Successfully parses a massively complex, deeply nested UI component tree', () => {
			// In tagged template literals, the `parts` array is strictly the strings BETWEEN expressions.
			// Every comma here represents exactly ONE `${variable}` interpolation.
			const parts = [
				'<div id="app-root" class=',
				'data-theme="dark"><header><nav><a href="/" active>',
				'</a><ul><li><a href="/about">About</a></li></ul><',
				'></nav></header><main data-id=',
				'><section   ><h1>Welcome to ',
				'</h1><p>Your daily dose of code.</p><hr><img src=',
				' alt="Hero Image" /></section><div class="grid layout-3-col" style=',
				'><Card title="Post 1"><br><input type="text" disabled /></Card><Card title="Post 2"><button @click=',
				' class   =   "btn-primary">Click Me ',
				'</button></Card></div></main><footer><div class="copy">&copy; 2025 ',
				'</div></footer></div>',
			];

			const ast = parse(parts);

			// Helper to safely grab elements while ignoring text nodes
			const getTags = (el: any) =>
				el.children.filter((c: any) => typeof c === 'object' && 'tag' in c);

			// Root Validation
			expect(ast.tag).toBe('html');
			const rootDiv = getTags(ast)[0];
			expect(rootDiv.tag).toBe('div');
			expect(rootDiv.attrs).toEqual([
				{ attr: 'id', value: 'app-root' },
				{ attr: 'class', value: 0 }, // interpolated class
				{ attr: 'data-theme', value: 'dark' },
			]);

			const rootTags = getTags(rootDiv);
			const header = rootTags[0];
			const main = rootTags[1];
			const footer = rootTags[2];

			// Header & DO-tag Validation
			expect(header.tag).toBe('header');
			const nav = getTags(header)[0];
			expect(nav.tag).toBe('nav');
			const navA = getTags(nav)[0];
			expect(navA.children[0]).toBe(1); // Interpolated text in <a>

			// Checks the <${Component}> syntax (DO-tag)
			const doTag = nav.children.find((c: any) => typeof c === 'object' && c.type === 'do');
			expect(doTag).toEqual({ type: 'do', arg: 2 });

			// Main Validation
			expect(main.tag).toBe('main');
			expect(main.attrs).toEqual([{ attr: 'data-id', value: 3 }]);

			const mainTags = getTags(main);
			const section = mainTags[0];
			const grid = mainTags[1];

			// Section Hero Validation
			expect(getTags(section)[0].tag).toBe('h1');
			expect(getTags(section)[0].children).toEqual(['Welcome to ', 4]); // Text + Interpolation

			expect(getTags(section)[2].tag).toBe('hr'); // Auto-closed void tag

			const img = getTags(section)[3];
			expect(img.tag).toBe('img'); // Self-closed void tag
			expect(img.attrs[0]).toEqual({ attr: 'src', value: 5 });

			// Grid Validation
			expect(grid.attrs[1]).toEqual({ attr: 'style', value: 6 });

			const cards = getTags(grid);
			expect(cards).toHaveLength(2);

			const button = getTags(cards[1])[0];
			expect(button.tag).toBe('button');
			expect(button.attrs[0]).toEqual({ attr: '@click', value: 7 });
			expect(button.children).toEqual(['Click Me ', 8]); // Text + Interpolation

			// Footer Validation
			expect(footer.tag).toBe('footer');
			const copyDiv = getTags(footer)[0];
			expect(copyDiv.children).toEqual(['&copy; 2025 ', 9]);
		});
	});
});

describe('8. Fuzz Testing', () => {
	it('should survive 10,000 random mutations without infinite loops or TypeErrors', () => {
		// If this test times out, the parser has an infinite loop bug!
		const ITERATIONS = 10000;

		// A specific charset designed to trigger parser state changes
		const charset = '<>="/a1 \n\t\'!-';

		let successfulParses = 0;
		let syntaxErrors = 0;

		for (let i = 0; i < ITERATIONS; i++) {
			// 1. Generate a random number of template "parts" (1 to 5 parts)
			const numParts = Math.floor(Math.random() * 5) + 1;
			const parts: string[] = [];

			for (let p = 0; p < numParts; p++) {
				// 2. Generate random strings for each part (0 to 30 chars long)
				const partLen = Math.floor(Math.random() * 30);
				let str = '';
				for (let c = 0; c < partLen; c++) {
					str += charset[Math.floor(Math.random() * charset.length)];
				}
				parts.push(str);
			}

			// 3. Occasionally inject intentional edge cases into the parts
			if (i % 10 === 0) parts.push('<!--'); // dangling comment
			if (i % 11 === 0) parts[0] = '<' + parts[0]; // sudden open
			if (i % 12 === 0) parts[0] = parts[0] + '="'; // dangling string
			if (i % 13 === 0) parts.push('/'); // standalone slash

			try {
				// Attempt to parse the absolute garbage
				parse(parts);
				successfulParses++;

				// If it didn't throw, it MUST be valid AST (mostly empty or text arrays)
			} catch (error) {
				syntaxErrors++;

				// 4. THE CRITICAL ASSERTION
				// If the parser fails, it MUST be a managed SyntaxError.
				// If it's a TypeError (e.g. reading properties of undefined),
				// it means our element stack broke or regex failed badly.
				if (!(error instanceof SyntaxError)) {
					console.error('CRITICAL FAILURE ON PARTS:', parts);
					console.error(error);
				}
				expect(error).toBeInstanceOf(SyntaxError);
			}
		}

		// Just to log the results so you can see it's actually doing work
		console.log(
			`Fuzz test completed. Valid parses: ${successfulParses}, SyntaxErrors caught: ${syntaxErrors}`,
		);
	});

	it('should survive structure-aware fuzzing (mangled valid HTML)', () => {
		const baseHTML =
			'<div id="app" data-value=a><span class=\'text\'>Hello </span><img /></div>';

		for (let i = 0; i < 5000; i++) {
			// Randomly slice, slice out, or duplicate chunks of the valid HTML
			const start = Math.floor(Math.random() * baseHTML.length);
			const end = start + Math.floor(Math.random() * (baseHTML.length - start));

			const mangled = baseHTML.slice(0, start) + baseHTML.slice(end);
			const parts = [mangled]; // Single string array

			try {
				parse(parts);
			} catch (error) {
				expect(error).toBeInstanceOf(SyntaxError);
			}
		}
	});
});

describe('9. Generative Property-Based Fuzzing (Valid HTML)', () => {
	// Predefined safe data for generation
	const voidTags = ['img', 'br', 'hr', 'input', 'meta', 'link'];
	const normalTags = ['div', 'span', 'a', 'p', 'section', 'b', 'custom-component', 'svg'];
	const attrNames = ['id', 'class', 'data-value', 'aria-hidden', '@click', ':style'];
	const safeText = ['hello', 'world', ' text ', '123', '\n\t', '🚀'];

	// Random helpers
	const rand = (arr: any[]) => arr[Math.floor(Math.random() * arr.length)];
	const randBool = () => Math.random() > 0.5;
	const randWhitespace = () => rand(['', ' ', '\n', '\t', '  ']);

	it('should perfectly reconstruct 1,000 randomly generated valid HTML structures', () => {
		const ITERATIONS = 1000;

		for (let i = 0; i < ITERATIONS; i++) {
			let parts: string[] = [''];
			let varCount = 0;

			// Appends string to the current template part
			const append = (str: string) => {
				parts[parts.length - 1] += str;
			};

			// Triggers a variable interpolation (${var})
			const interpolate = () => {
				const id = varCount++;
				parts.push(''); // Break the string here to simulate literal interpolation
				return id;
			};

			// Recursively generates an element and its expected AST node
			const generateElement = (depth: number): Element => {
				const isVoid = randBool();
				const tag = isVoid ? rand(voidTags) : rand(normalTags);

				append('<' + tag);

				const attrs: { attr: string; value: string | number }[] = [];
				const numAttrs = Math.floor(Math.random() * 10);

				for (let a = 0; a < numAttrs; a++) {
					append(' '); // Ensure space after tag name or previous attr
					append(randWhitespace());

					const attrName = rand(attrNames) + '-' + a; // Keep unique-ish
					append(attrName);
					append(randWhitespace());

					let value: string | number = '';
					const attrType = Math.floor(Math.random() * 5);

					if (attrType === 0) {
						// Boolean attribute (no "=")
					} else if (attrType === 1) {
						// Unquoted attribute
						append('=');
						append(randWhitespace());
						value = rand(['val1', 'val2', '123']);
						append(value as string);
					} else if (attrType === 2) {
						// Single quoted
						append('=');
						append(randWhitespace());
						value = rand(['val 1', 'val 2', '']);
						append(`'${value}'`);
					} else if (attrType === 3) {
						// Double quoted
						append('=');
						append(randWhitespace());
						value = rand(['val 1', 'val 2', '']);
						append(`"${value}"`);
					} else if (attrType === 4) {
						// Interpolated variable
						append('=');
						append(randWhitespace());
						value = interpolate();
					}
					attrs.push({ attr: attrName, value });
				}

				append(randWhitespace());

				if (isVoid) {
					if (randBool()) append('/'); // Optionally self-close
					append('>');
					return { tag, attrs, children: [] };
				} else {
					append('>');
					const children: any[] = [];

					// Limit depth to prevent stack overflow
					const numChildren = depth < 8 ? Math.floor(Math.random() * 8) : 0;
					for (let c = 0; c < numChildren; c++) {
						const child = generateNode(depth + 1);
						if (child !== null) children.push(child);
					}

					append(`</${tag}>`);
					return { tag, attrs, children };
				}
			};

			// Generates any valid node (Text, Interpolation, DO-tag, Comment, or Element)
			const generateNode = (depth: number): any => {
				const type = Math.floor(Math.random() * 6);

				if (type === 0 || type === 1) {
					// 33% chance: Text node
					const txt = rand(safeText);
					append(txt);
					return txt;
				} else if (type === 2) {
					// 16% chance: Interpolated content
					return interpolate();
				} else if (type === 3) {
					// 16% chance: DO-tag (<${Component} />)
					append('<');
					const id = interpolate();
					append('>');
					return { type: 'do', arg: id };
				} else if (type === 4) {
					// 16% chance: Comment (ignored by parser)
					append('<!-- ' + rand(safeText) + ' -->');
					return null;
				} else {
					// 16% chance: Child Element
					return generateElement(depth);
				}
			};

			// --- START GENERATING ROOT ---
			const expectedChildren: any[] = [];
			const numRootChildren = Math.floor(Math.random() * 3) + 1;

			for (let c = 0; c < numRootChildren; c++) {
				const child = generateNode(0);
				if (child !== null) expectedChildren.push(child);
			}

			const cleanChildren = (children: any[]): any[] => {
				const res: any[] = [];
				for (const c of children) {
					if (typeof c === 'string') {
						if (c === '') continue; // Parser ignores empty strings
						if (typeof res[res.length - 1] === 'string') {
							res[res.length - 1] += c; // Merge adjacent strings
						} else {
							res.push(c);
						}
					} else if (c && typeof c === 'object' && c.tag) {
						c.children = cleanChildren(c.children);
						res.push(c);
					} else {
						res.push(c);
					}
				}
				return res;
			};

			const expectedAST = {
				tag: 'html',
				attrs: [],
				children: cleanChildren(expectedChildren),
			};

			// --- ASSERTION ---
			try {
				const parsedAST = parse(parts);
				expect(parsedAST).toEqual(expectedAST);
			} catch (error) {
				// If it fails, print the exact parts array that broke it so you can fix it!
				console.error(`\n❌ FUZZ TEST FAILED ON ITERATION ${i}`);
				console.error('Generated Parts Array:', parts);
				console.error('Expected AST:', JSON.stringify(expectedAST, null, 2));
				throw error;
			}
		}
	});
});
