import { Context, render_list_enumerated } from '../../src/index.ts';

interface TestItem {
	id: number;
	value: string;
}

// Actions our fuzzer can perform
type FuzzAction =
	| { type: 'APPEND'; items: TestItem[] }
	| { type: 'PREPEND'; items: TestItem[] }
	| { type: 'INSERT'; index: number; items: TestItem[] }
	| { type: 'REMOVE'; index: number; count: number }
	| { type: 'SWAP'; indexA: number; indexB: number }
	| { type: 'MOVE'; from: number; to: number }
	| { type: 'UPDATE'; index: number; newValue: string; new_id: number }
	| { type: 'REVERSE' }
	| { type: 'SHUFFLE' }
	| { type: 'CLEAR' }
	| { type: 'BATCH_REPLACE'; items: TestItem[] };

export async function run_render_list_fuzzer(iterations: number = 1000) {
	console.time('fuzz');
	console.log(
		`🚀 Starting large-scale fuzz test for render_list_enumerated (${iterations} iterations)...`,
	);

	// --- 1. Setup Framework and DOM Environment ---
	const rootEl = document.createElement('div');
	rootEl.id = 'fuzz-root';
	document.body.appendChild(rootEl);

	const ctx = new Context(rootEl);
	const build = ctx.root_chunk();
	const { html, signal } = build;

	// The source of truth for our fuzzer
	let expectedState: TestItem[] = [];

	// The reactive signal driving the framework
	const listSignal = signal<TestItem[]>([]);

	// A monotonic counter to guarantee unique IDs for keyed rendering
	let idCounter = 0;

	// --- 2. Mount the Component ---
	// We mount a container and use `render_list_enumerated`.
	// We encode the ID, Value, and Index into the DOM attributes/text to assert later.
	html`
		<div id="list-container">
			<div>
				<${render_list_enumerated(
					listSignal,
					(item) => item.id, // KEY FUNCTION
					'div', // TAG
					(build, item, indexSignal) => {
						// We use HTML data attributes to easily query and verify state in the DOM
						build.html`
						<div 
							class="fuzz-item"
							data-id="${item.id}"
							data-val="${item.value}"
							data-idx="${indexSignal}" 
						>
							ID:${item.id}|VAL:${item.value}|IDX:${indexSignal}
						</div>
					`;
					},
				)} />
			</div>
		</div>
	`;

	// --- 3. Fuzzer Utilities ---
	const actionHistory: FuzzAction[][] = [];

	const createItems = (count: number): TestItem[] => {
		return Array.from({ length: count }, () => ({
			id: ++idCounter,
			value: `val_${Math.random().toString(36).slice(2, 7)}`,
		}));
	};

	const randomInt = (min: number, max: number) =>
		Math.floor(Math.random() * (max - min + 1)) + min;

	// --- 4. The Fuzzing Loop ---
	for (let i = 0; i < iterations; i++) {
		const actions: FuzzAction[] = [];
		for (let j = 0; j < 100; j++) {
			const action = generateRandomAction(
				expectedState.length,
				createItems,
				randomInt,
				() => ++idCounter,
			);
			actions.push(action);
			applyActionToState(expectedState, action);
		}
		actionHistory.push(actions);

		try {
			// Apply mutation to the Reactive Signal (cloning to trigger reactivity)
			listSignal.value = [...expectedState];

			// Yield to event loop / microtask queue to allow framework to update the DOM
			ctx.store.flush_updates();

			// --- 5. INVARIANT VERIFICATION ---
			verifyDOM(rootEl, expectedState);
		} catch (error) {
			console.error(`❌ Fuzz test failed at iteration ${i}!`);
			console.error('Last 10 Actions leading to failure:');
			console.dir(actionHistory.slice(-10), { depth: null });
			console.error('Expected State:', expectedState);
			console.error('DOM State HTML:', rootEl.innerHTML);
			throw error;
		}
	}

	console.log(
		`✅ Fuzz test completed successfully! Passed ${iterations} iteration (100 mutation each).`,
	);
	console.log('Action History:', actionHistory);
	console.timeEnd('fuzz');

	document.querySelector('#main')?.append(rootEl);
}

// --- Fuzzer Mutation Logic ---

function generateRandomAction(
	currentLength: number,
	createItems: (n: number) => TestItem[],
	randomInt: (min: number, max: number) => number,
	next_id: () => number,
): FuzzAction {
	const rand = Math.random();

	if (currentLength === 0 || rand < 0.1)
		return { type: 'APPEND', items: createItems(randomInt(1, 10)) };
	if (rand < 0.2) return { type: 'PREPEND', items: createItems(randomInt(1, 5)) };
	if (rand < 0.4)
		return {
			type: 'INSERT',
			index: randomInt(0, currentLength),
			items: createItems(randomInt(1, 10)),
		};
	if (rand < 0.5)
		return {
			type: 'REMOVE',
			index: randomInt(0, currentLength - 1),
			count: randomInt(1, Math.min(4, currentLength)),
		};
	if (rand < 0.6 && currentLength >= 2)
		return {
			type: 'SWAP',
			indexA: randomInt(0, currentLength - 1),
			indexB: randomInt(0, currentLength - 1),
		};
	if (rand < 0.7 && currentLength >= 2)
		return {
			type: 'MOVE',
			from: randomInt(0, currentLength - 1),
			to: randomInt(0, currentLength - 1),
		};
	if (rand < 0.8)
		return {
			type: 'UPDATE',
			index: randomInt(0, currentLength - 1),
			new_id: next_id(),
			newValue: `updated_${Math.random().toString(36).slice(2)}`,
		};
	if (rand < 0.85) return { type: 'REVERSE' };
	if (rand < 0.95) return { type: 'SHUFFLE' };
	if (rand < 0.98) return { type: 'BATCH_REPLACE', items: createItems(randomInt(0, 30)) };
	return { type: 'CLEAR' };
}

function applyActionToState(state: TestItem[], action: FuzzAction) {
	switch (action.type) {
		case 'APPEND':
			state.push(...action.items);
			break;
		case 'PREPEND':
			state.unshift(...action.items);
			break;
		case 'INSERT':
			state.splice(action.index, 0, ...action.items);
			break;
		case 'REMOVE':
			state.splice(action.index, action.count);
			break;
		case 'SWAP':
			const temp = state[action.indexA];
			state[action.indexA] = state[action.indexB];
			state[action.indexB] = temp;
			break;
		case 'MOVE':
			const [moved] = state.splice(action.from, 1);
			state.splice(action.to, 0, moved);
			break;
		case 'UPDATE':
			// Clone object to ensure reference equality changes if framework relies on it
			state[action.index] = {
				...state[action.index],
				id: action.new_id,
				value: action.newValue,
			};
			break;
		case 'REVERSE':
			state.reverse();
			break;
		case 'SHUFFLE':
			for (let i = state.length - 1; i > 0; i--) {
				const j = Math.floor(Math.random() * (i + 1));
				[state[i], state[j]] = [state[j], state[i]];
			}
			break;
		case 'CLEAR':
			state.length = 0;
			break;
		case 'BATCH_REPLACE':
			state.length = 0;
			state.push(...action.items);
			break;
	}
}

// --- DOM Verification Logic ---

function verifyDOM(rootEl: HTMLElement, expectedState: TestItem[]) {
	// 1. Find the wrapper div created by render_list_enumerated
	const container = rootEl.querySelector('#list-container > div'); // Adjust selector based on actual output structure
	if (!container) throw new Error('List container not found in DOM.');

	// We only look at elements with the class 'fuzz-item' (ignoring framework comment nodes/anchors)
	const domNodes = Array.from(container.querySelectorAll('.fuzz-item')) as HTMLElement[];

	// Check 1: Length match
	if (domNodes.length !== expectedState.length) {
		throw new Error(
			`Length mismatch! Expected ${expectedState.length}, got ${domNodes.length} DOM nodes.`,
		);
	}

	// Check 2: Sequential verification of Data, Keys, and Indexes
	for (let i = 0; i < expectedState.length; i++) {
		const node = domNodes[i];
		const expected = expectedState[i];

		const actualId = parseInt(node.getAttribute('data-id') || '-1', 10);
		const actualVal = node.getAttribute('data-val');
		const actualIdx = parseInt(node.getAttribute('data-idx') || '-1', 10);

		// A. Validate ID (Key function integrity)
		if (actualId !== expected.id) {
			throw new Error(
				`ID Mismatch at index ${i}. Expected ID ${expected.id}, found ID ${actualId}. Node diffing failed.`,
			);
		}

		// B. Validate Value (Data rendering integrity)
		if (actualVal !== expected.value) {
			throw new Error(
				`Value Mismatch at index ${i}. Expected "${expected.value}", found "${actualVal}".`,
			);
		}

		// C. Validate Enumerated Index (CRITICAL for render_list_enumerated)
		// This tests if `index: Signal<number>` successfully triggered updates when items shifted positions.
		if (actualIdx !== i) {
			throw new Error(
				`Index Signal Failure at real index ${i}. The item with ID ${expected.id} still thinks its index is ${actualIdx}. Ensure the index signal updates during moves/inserts.`,
			);
		}

		// D. Validate Text Content (ensuring HTML template parsed correctly)
		const expectedText = `ID:${expected.id}|VAL:${expected.value}|IDX:${i}`;
		if (!node.textContent?.includes(expectedText)) {
			throw new Error(
				`Text Content Mismatch at index ${i}. Expected to contain "${expectedText}", but got "${node.textContent}".`,
			);
		}
	}
}

// Utility to allow DOM updates to process.
// Standard for frameworks batching reactivity via microtasks (like queueMicrotask or Promise.resolve).
function microtask(): Promise<void> {
	return new Promise((resolve) => queueMicrotask(resolve));
}
