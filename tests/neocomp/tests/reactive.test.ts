import { describe, expect, it, vi } from 'vitest';
import { ROSignal, Signal, type SlabID, Store } from '../../src/reactive.ts'; // Adjust import based on filename

describe('Store Core Functionality', () => {
	it('creates and reads props directly', () => {
		const store = new Store();
		const propId = store.prop(42);
		expect(store.get(propId)).toBe(42);
		expect(store.peek(propId)).toBe(42);
	});

	it('sets and updates props', () => {
		const store = new Store();
		const propId = store.prop(10);

		store.set(propId, 20);
		expect(store.get(propId)).toBe(20);

		store.update(propId, (v) => v * 2);
		expect(store.get(propId)).toBe(40);
	});
});

describe('Signals API', () => {
	it('wraps props in Signals', () => {
		const store = new Store();
		const sig = store.signal('hello');

		expect(sig.value).toBe('hello');
		expect(sig.peek()).toBe('hello');

		sig.value = 'world';
		expect(sig.value).toBe('world');

		sig.update((v) => v + '!');
		expect(sig.value).toBe('world!');
	});

	it('creates signal_for existing prop', () => {
		const store = new Store();
		const prop = store.prop(99);
		const sig = store.signal_for(prop);

		expect(sig.value).toBe(99);
	});

	it('provides Read-Only Signals correctly', () => {
		const store = new Store();
		const sig = store.signal(1);
		const ro = sig.as_ro();

		expect(ro.value).toBe(1);
		expect(ro.peek()).toBe(1);
		expect(ro).toBeInstanceOf(ROSignal);
	});
});

describe('Reactivity & Effects', () => {
	it('runs effects immediately on creation', () => {
		const store = new Store();
		const sig = store.signal(10);
		let runs = 0;

		store.effect(() => {
			sig.value; // Read to track
			runs++;
		});

		expect(runs).toBe(1);
	});

	it('triggers effects after flush_updates', () => {
		const store = new Store();
		const sig = store.signal(10);
		let val = 0;

		store.effect(() => {
			val = sig.value;
		});

		sig.value = 20;
		expect(val).toBe(10); // Not flushed yet

		store.flush_updates();
		expect(val).toBe(20); // Flushed
	});

	it('does not trigger if prop is peeked (untracked)', () => {
		const store = new Store();
		const sig = store.signal(10);
		let runs = 0;

		store.effect(() => {
			sig.peek(); // Peek avoids tracking
			runs++;
		});

		sig.value = 20;
		store.flush_updates();

		expect(runs).toBe(1); // Still 1
	});

	it('force_update triggers flush regardless of value change', () => {
		const store = new Store();
		const sig = store.signal(10);
		let runs = 0;

		store.effect(() => {
			sig.value;
			runs++;
		});

		store.force_update(sig.id);
		store.flush_updates();

		expect(runs).toBe(2);
	});
});

describe('Computed Signals', () => {
	it('creates a read-only computed derived from other signals', () => {
		const store = new Store();
		const a = store.signal(2);
		const b = store.signal(3);
		const comp = store.computed(() => a.value * b.value);

		expect(comp.value).toBe(6);

		a.value = 4;
		store.flush_updates();
		expect(comp.value).toBe(12);
	});

	it('throws if a computed tries to write to a property', () => {
		const store = new Store();
		const a = store.signal(1);
		const b = store.signal(2);

		expect(() => {
			store.computed(() => {
				b.value = 10; // Write during computed
				return a.value;
			});
		}).toThrow('computed properties cannot set other properties');
	});
});

describe('Topological Sort & Diamond Dependency', () => {
	it('solves the diamond dependency problem smoothly', () => {
		// Graph:
		// Root -> Left -> Combined
		// Root -> Right -> Combined
		const store = new Store();
		const root = store.signal(1);

		const left = store.computed(() => root.value * 2);
		const right = store.computed(() => root.value * 3);

		let runs = 0;
		const combined = store.computed(() => {
			runs++;
			return left.value + right.value;
		});

		expect(combined.value).toBe(5);
		expect(runs).toBe(1);

		// Updating root should only trigger `combined` ONCE
		root.value = 2;
		store.flush_updates();

		expect(combined.value).toBe(10); // 4 + 6
		expect(runs).toBe(2); // Only evaluated once during flush
	});
});

describe('Error Handling & Edge Cases', () => {
	it('throws on circular dependencies', () => {
		const store = new Store();
		const a = store.signal(1);
		const b = store.signal(2);

		// Intentionally create a cycle via manual effects
		// Effect 1: Reads A, Writes B
		store.effect(() => {
			b.value = a.value + 1;
		});
		// Effect 2: Reads B, Writes A
		store.effect(() => {
			a.value = b.value + 1;
		});

		// Trigger the cycle
		a.value = 10;
		expect(() => store.flush_updates()).toThrow('detected circular dependency');
	});

	it('throws when tracking is nested', () => {
		const store = new Store();
		store.start_track();
		expect(() => store.start_track()).toThrow('already tracking');
		store.end_track();
	});

	it('throws when ending track without starting', () => {
		const store = new Store();
		expect(() => store.end_track()).toThrow('not tracking');
	});

	it('allows effect_manual without init_run', () => {
		const store = new Store();
		const mock = vi.fn();
		store.effect_manual([], [], mock, undefined, false);
		expect(mock).not.toHaveBeenCalled();
	});
});

describe('Slab Allocator (Memory Management)', () => {
	it('assigns props and effects to a slab, and cleans them up', () => {
		const store = new Store();
		const slab = store.new_slab();

		expect(store.has_slab(slab)).toBe(true);

		const sig = store.signal(5, slab);
		let runs = 0;
		store.effect(() => {
			sig.value;
			runs++;
		}, slab);

		sig.value = 10;
		store.flush_updates();
		expect(runs).toBe(2);

		// Remove slab
		store.remove_slab(slab);
		expect(store.has_slab(slab)).toBe(false);

		// The property and effect should be destroyed
		expect(store.peek(sig.id)).toBeUndefined();

		// Changing it shouldn't trigger the destroyed effect
		sig.value = 20;
		store.flush_updates();
		expect(runs).toBe(2); // Unchanged
	});

	it('defers slab removal if currently updating', () => {
		const store = new Store();
		const slab = store.new_slab();
		const trigger = store.signal(1);

		let removed = false;

		store.effect(() => {
			trigger.value;
			// Trying to remove slab during flush phase
			if (store.updating) {
				store.remove_slab(slab);
				removed = true;
			}
		});

		trigger.value = 2;
		store.flush_updates();

		expect(removed).toBe(true);
		expect(store.has_slab(slab)).toBe(false); // Successfully deferred and removed
	});

	it('throws if removing non-existent slab', () => {
		const store = new Store();
		expect(() => store.remove_slab(999 as any)).toThrow('slab 999 does not exist');
	});
});

describe('Fuzzing & Deep Graphs', () => {
	it('handles a heavily chained dependency line correctly', () => {
		const store = new Store();
		const signals = [store.signal(0)];

		// Create a chain of 100 computeds: sig[i] = sig[i-1] + 1
		for (let i = 1; i <= 100; i++) {
			const prev = signals[i - 1];
			signals.push(store.computed(() => prev.value + 1) as any);
		}

		expect(signals[100].value).toBe(100);

		// Updating the root should cascade all the way down
		signals[0].value = 50;
		store.flush_updates();

		expect(signals[100].value).toBe(150);
	});

	it('fuzz testing: random graph with flush consistency', () => {
		const store = new Store();
		const nodes: any[] = [];

		// Generate 10 base nodes
		for (let i = 0; i < 10; i++) {
			nodes.push(store.signal(1));
		}

		// Generate 40 derived nodes
		for (let i = 10; i < 50; i++) {
			// Randomly pick 2 distinct previous nodes
			const idx1 = Math.floor(Math.random() * i);
			let idx2 = Math.floor(Math.random() * i);
			while (idx1 === idx2) idx2 = Math.floor(Math.random() * i);

			nodes.push(store.computed(() => nodes[idx1].value + nodes[idx2].value));
		}

		// Mutate base nodes multiple times
		for (let f = 0; f < 5; f++) {
			for (let i = 0; i < 5; i++) {
				nodes[i].value = Math.floor(Math.random() * 100);
			}
			expect(() => store.flush_updates()).not.toThrow();
			// Verify final node has a valid number
			expect(typeof nodes[49].value).toBe('number');
			expect(Number.isNaN(nodes[49].value)).toBe(false);
		}
	});
});

// A simple seeded random number generator for reproducible fuzzing
class SeededRNG {
	private seed: number;
	constructor(seed: number) {
		this.seed = seed;
	}
	next(): number {
		this.seed = (this.seed * 16807) % 2147483647;
		return (this.seed - 1) / 2147483646;
	}
	nextInt(max: number): number {
		return Math.floor(this.next() * max);
	}
	pick<T>(arr: T[]): T {
		return arr[this.nextInt(arr.length)];
	}
}

describe('Advanced Fuzzing & Chaos', () => {
	it('fuzzes a massive, deeply connected DAG and verifies outputs via an Oracle', () => {
		// Oracle testing: We build a reactive graph, and a plain-JS parallel graph.
		// We mutate the reactive base, mutate the plain-JS base, evaluate both, and compare.
		const store = new Store();
		const rng = new SeededRNG(1337); // Fixed seed for reproducible test

		const NUM_BASE = 50;
		const NUM_COMPUTEDS = 500;
		const NUM_ITERATIONS = 20;

		// 1. Setup Base Nodes
		const reactiveBases: Signal<number>[] = [];
		const oracleBases: number[] = [];

		for (let i = 0; i < NUM_BASE; i++) {
			const initialVal = rng.nextInt(100);
			reactiveBases.push(store.signal(initialVal));
			oracleBases.push(initialVal);
		}

		// 2. Setup Computed Nodes & Oracle Evaluators
		// We ensure a DAG by only allowing node N to depend on nodes 0 to N-1
		type OracleOp = { type: 'ADD' | 'SUB' | 'MUL'; deps: number[] };
		const allReactiveNodes: ROSignal<number>[] = [...reactiveBases];
		const oracleOps: (OracleOp | null)[] = Array(NUM_BASE).fill(null);

		for (let i = 0; i < NUM_COMPUTEDS; i++) {
			const opType = rng.pick(['ADD', 'SUB', 'MUL']);
			const numDeps = rng.nextInt(3) + 2; // 2 to 4 dependencies
			const deps: number[] = [];

			for (let d = 0; d < numDeps; d++) {
				// Only pick from already created nodes to strictly prevent cycles
				deps.push(rng.nextInt(allReactiveNodes.length));
			}

			oracleOps.push({ type: opType as any, deps });

			const computed = store.computed(() => {
				let result = opType === 'MUL' ? 1 : 0;
				for (let d of deps) {
					const val = allReactiveNodes[d].value;
					if (opType === 'ADD') result += val;
					if (opType === 'SUB') result -= val;
					if (opType === 'MUL') result *= val % 10; // keep numbers from overflowing infinity
				}
				return result;
			});

			allReactiveNodes.push(computed);
		}

		// Oracle Evaluation Engine
		const evaluateOracle = () => {
			const results = [...oracleBases];
			for (let i = NUM_BASE; i < oracleOps.length; i++) {
				const op = oracleOps[i]!;
				let result = op.type === 'MUL' ? 1 : 0;
				for (let d of op.deps) {
					const val = results[d];
					if (op.type === 'ADD') result += val;
					if (op.type === 'SUB') result -= val;
					if (op.type === 'MUL') result *= val % 10;
				}
				results.push(result);
			}
			return results;
		};

		// 3. Fuzzing Loop
		for (let iteration = 0; iteration < NUM_ITERATIONS; iteration++) {
			// Mutate a random subset of base nodes
			const mutations = rng.nextInt(10) + 1;
			for (let m = 0; m < mutations; m++) {
				const targetBase = rng.nextInt(NUM_BASE);
				const newVal = rng.nextInt(1000);

				reactiveBases[targetBase].value = newVal;
				oracleBases[targetBase] = newVal;
			}

			// Flush reactive system
			expect(() => store.flush_updates()).not.toThrow();

			// Evaluate Oracle Truth
			const oracleResults = evaluateOracle();

			// 4. Assert correctness across ALL 550 nodes
			for (let i = 0; i < allReactiveNodes.length; i++) {
				expect(allReactiveNodes[i].peek()).toBe(oracleResults[i]);
			}
		}
	});

	it('chaos testing: interlaced slab allocation, mutation, and garbage collection', () => {
		const store = new Store();
		const rng = new SeededRNG(42);

		let activeSlabs: SlabID[] = [];
		let globalRunCounter = 0;

		// Represents a reference to an effect and the signal it tracks
		type TrackedSlab = {
			id: SlabID;
			sig: Signal<number>;
			runs: number;
		};
		const trackedNodes: TrackedSlab[] = [];

		// Run 100 chaotic actions
		for (let i = 0; i < 100; i++) {
			const action = rng.pick([
				'CREATE_SLAB',
				'ADD_EFFECT',
				'MUTATE',
				'DELETE_SLAB',
				'FLUSH',
			]);

			if (action === 'CREATE_SLAB' || activeSlabs.length === 0) {
				activeSlabs.push(store.new_slab());
			} else if (action === 'ADD_EFFECT') {
				const slabId = rng.pick(activeSlabs);
				const sig = store.signal(rng.nextInt(100), slabId);
				const tracker = { id: slabId, sig, runs: 0 };

				store.effect(() => {
					sig.value; // Read to track
					tracker.runs++;
					globalRunCounter++;
				}, slabId);

				trackedNodes.push(tracker);
			} else if (action === 'MUTATE' && trackedNodes.length > 0) {
				// Mutate a random signal (could be in a live or deleted slab!)
				const target = rng.pick(trackedNodes);
				target.sig.value = rng.nextInt(100);
			} else if (action === 'DELETE_SLAB' && activeSlabs.length > 0) {
				const idx = rng.nextInt(activeSlabs.length);
				const slabId = activeSlabs[idx];

				store.remove_slab(slabId);
				activeSlabs.splice(idx, 1); // remove from our active list
			} else if (action === 'FLUSH') {
				// Should never crash, even if we mutated a signal whose slab/effect was deleted
				expect(() => store.flush_updates()).not.toThrow();
			}
		}

		// Final flush to settle state
		store.flush_updates();

		// Ensure nodes in deleted slabs stop reacting
		const deletedNodes = trackedNodes.filter((n) => !activeSlabs.includes(n.id));

		// Snapshot their run counts
		const snapshot = deletedNodes.map((n) => n.runs);

		// Mutate them all
		deletedNodes.forEach((n) => {
			n.sig.value = 999;
		});
		store.flush_updates();

		// Verify run counts did NOT increase for deleted slabs
		deletedNodes.forEach((n, idx) => {
			expect(n.runs).toBe(snapshot[idx]);
		});
	});

	it('fuzzes and aggressively detects hidden circular dependencies', () => {
		const store = new Store();
		const rng = new SeededRNG(999);

		const nodes: Signal<number>[] = [];
		for (let i = 0; i < 20; i++) nodes.push(store.signal(0));

		// Create random strict DAG connections via manual effects
		for (let i = 1; i < 20; i++) {
			const depsCount = rng.nextInt(3) + 1;
			const readProps = [];
			for (let d = 0; d < depsCount; d++) {
				readProps.push(nodes[rng.nextInt(i)].id); // Only depend on previously created nodes
			}

			store.effect_manual(
				readProps, // Reads strictly backwards
				[nodes[i].id], // Writes to current
				() => {
					nodes[i].value = nodes[i].peek() + 1;
				},
			);
		}

		store.flush_updates(); // Should be completely fine.

		// Introduce Chaos: Randomly pick an older node and make it depend on a newer node
		const olderNodeIdx = rng.nextInt(10);
		const newerNodeIdx = olderNodeIdx + rng.nextInt(9) + 1; // strictly newer

		// We force a cycle: Older reads Newer, Writes Older. (Newer already relies on Older via DAG).
		store.effect_manual(
			[nodes[newerNodeIdx].id], // Reads newer!
			[nodes[olderNodeIdx].id], // Writes older!
			() => {
				nodes[olderNodeIdx].value = nodes[newerNodeIdx].peek() + 1;
			},
			undefined,
			false, // don't init run
		);

		// Mutate the older node to trigger the chain
		nodes[olderNodeIdx].value = 99;

		// Assert that the store catches the cycle and throws
		expect(() => store.flush_updates()).toThrow('detected circular dependency');
	});
});
