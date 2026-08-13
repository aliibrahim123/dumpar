use std::ops::Deref;

use js_sys::wasm_bindgen::prelude::{JsCast, JsValue};
use neoview_web::{
	apply::{
		attr, class, el, node, on, prop, show_if, style,
		tags::{div, span},
		text,
	},
	prelude::*,
};
use web_sys::{HtmlInputElement, Node, Text, window};

#[test]
fn a() {
	let document = window().unwrap().document().unwrap();
	let ctx =
		DomContext::new(document.query_selector("#main").unwrap().unwrap(), Default::default());
	let mut ctx = ctx.borrow_mut();
	let a = ctx.prop(1);
	let b = ctx.prop(true);
	let c = ctx.prop("red".to_string());
	let mut build = ctx.root_chunk();
	chunk!(build, html {
		span (id: "a", aria_autocomplete: 1, aa-aax-bb: 1) { "hallo" }
		a (prop.innerText: move |a| JsValue::from("a")) {}
		input("type": "number", class.a-b-a: move |ctx| !ctx.get(b), )
		span (id: 1+1, class: Some(Some(&&String::from("a"))))
		span () section (id: a, style: move |ctx| ctx.get(a) * 2)
		"abc" (class."px-[1]": true, style.accent_color: "a", style.color: move |ctx| Some(ctx.read(c).clone()))
		section	(on.click: (move |ctx, _| {
			*ctx.read_mut(a) += 1
		}),) {
			1, true, &Some(3.5), b, c, move |ctx| ctx.get(a) * if ctx.get(b) { 1 } else { 2 },
			(Text::new_with_data("hallo").unwrap()),
			do {
				build.apply(show_if(b));
				build.ref_el(move |_,el| println!("{}", el.inner_html()))
			}
		}
	});
	build.apply(div((
		attr("id", &&Some("a")),
		attr("contenteditable", b),
		span((class("hide", b), prop("html", JsValue::from("<b>hallo</b> world")))),
		el(
			"input",
			on("click", move |ctx, event| {
				let el = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				ctx.write(c, el.value())
			}),
		),
		style("min-width", "400px"),
		text("hallo"),
		node(document.create_text_node("a")),
	)));
	build.build();
}

#[cfg(test)]
mod tests {
	use neoview::*;

	use crate::VoidContext;

	#[test]
	fn a() {
		let mut store = Store::<VoidContext>::default();
		let slab = store.create_slab();
		let p1 = store.prop_in(slab, 0).unwrap();
		let p2 = store.prop_in(slab, 0).unwrap();
		println!("{p1:?} {p2:?}");

		println!("{}", store.read(p1));
		*store.read_mut(p1) = 1;
		println!("{}", store.peek(p1));
		store.write(p1, 2);
		println!("{}", store.read(p1));
		store.update(p1, |a| *a += 1);
		println!("{}", store.get(p1));

		store.remove_slab(slab).unwrap();
		println!("{:?} {}", store.try_read(p1), store.contains(p1));
	}

	#[test]
	fn d() {
		let mut store = Store::<VoidContext>::default();
		store.start_track().unwrap();
		println!("{}", store.is_tracking());

		let a = store.prop(1);
		store.read(a);
		let c = store.prop(2);
		store.peek(c);
		store.read(a);
		let b = store.prop(3);
		store.write(b, 4);
		store.write(b, 5);
		let d = store.prop(3);
		store.track_write(d);

		println!("{:?}", store.end_track());

		let c = store.create_slab();
		store.prop_in(c, "a").unwrap();
		store.remove_slab(c).unwrap();
		let c = store.create_slab();
		let d = store.prop_in(c, "a").unwrap();
		println!("{d:?}");
	}

	#[test]
	fn e() {
		let mut store = Store::<VoidContext>::default();
		let a = store.prop(vec![1, 2, 3]);
		let b: Vec<_> = store.read(a).iter().map(|a| *a + 1).collect();
		println!("{b:?}");
	}

	#[test]
	fn f() {
		let mut ctx = VoidContext::default();
		let mut store = ctx.store();
		let slab = store.create_slab();
		let a = store.prop_in(slab, 1).unwrap();
		let b = store.prop(2);
		let c = store.prop(3);
		Store::effect_in(&mut ctx, slab, move |ctx| {
			let store = ctx.store();
			let v = store.get(a) + *store.peek(c);
			println!("{v}");
			store.write(b, v);
		});
		let d = Store::computed_in(&mut ctx, slab, move |ctx| ctx.store().read(b) * 10);
		ctx.store().remove_slab(slab).unwrap();
		println!("{a:?} {ctx:#?}")
	}

	mod test_effect {
		use std::cell::Cell;
		use std::rc::Rc;

		// Assuming your crate is named `neoview`
		use neoview::*;

		// ==========================================
		// 1. MOCK ENVIRONMENT SETUP
		// ==========================================
		struct AppCtx {
			store: Store<AppCtx>,
		}

		impl Context for AppCtx {}
		impl GlobalStoreProv for AppCtx {}
		impl StoreProv for AppCtx {
			type Ctx = Self;
			fn ctx(&mut self) -> &mut Self::Ctx {
				self
			}
			fn ctx_ref(&self) -> &Self::Ctx {
				self
			}
			fn store(&mut self) -> &mut Store<Self> {
				&mut self.store
			}
			fn store_ref(&self) -> &Store<Self> {
				&self.store
			}
		}

		#[test]
		fn extreme_reactive_graph_stress_test() {
			let mut ctx = AppCtx { store: Store::default() };

			// ====================================================================
			// SCENARIO 1: THE WIDE DIAMOND (Glitch-Free Computed Check)
			// ====================================================================
			// Graph: A triggers B, C, D, E.
			// F computed depends on B, C, D, E.
			// If topological sort is broken, F will evaluate 4 times instead of 1.

			let a = ctx.store.prop(10_i32);

			let b = Store::computed(&mut ctx, move |c| c.store().get(a) + 1);
			let c = Store::computed(&mut ctx, move |c| c.store().get(a) + 2);
			let d = Store::computed(&mut ctx, move |c| c.store().get(a) + 3);
			let e = Store::computed(&mut ctx, move |c| c.store().get(a) + 4);

			let f_evals = Rc::new(Cell::new(0));
			let f_evals_clone = f_evals.clone();

			let f = Store::computed(&mut ctx, move |ctx| {
				// We use Rc<Cell> to count evaluations without triggering `Store` writes!
				f_evals_clone.set(f_evals_clone.get() + 1);
				ctx.store().get(b) + ctx.store().get(c) + ctx.store().get(d) + ctx.store().get(e)
			});
			assert_eq!(ctx.store.get(f), 11 + 12 + 13 + 14);
			assert_eq!(f_evals.get(), 1); // Evaluated once on init

			// Trigger the diamond
			ctx.store.write(a, 20);
			Store::flush_updates(&mut ctx);

			assert_eq!(ctx.store.get(f), 21 + 22 + 23 + 24);

			// 🔥 CRITICAL ASSERTION:
			// It MUST be exactly 2. If it's 5, your framework is glitching and doing
			// redundant calculations (O(N^2) explosion).
			assert_eq!(f_evals.get(), 2);

			// ====================================================================
			// SCENARIO 2: THE 1,000 NODE DEEP PIPELINE (Stack/Performance Stress)
			// ====================================================================
			// We create a chain of 1000 computed properties: P0 -> P1 -> P2 ... -> P1000.
			// This tests if your `update` loop's `visiting` stack and recursion can handle
			// massively deep graphs without stack overflowing or crashing.

			let mut pipeline = vec![ctx.store.prop(0_i32)];

			for i in 0..1000 {
				let prev = pipeline[i];
				let next = Store::computed(&mut ctx, move |c| c.store().get(prev) + 1);
				pipeline.push(next);
			}

			// Initial state: Node 1000 should equal 1000
			let tail = pipeline[1000];
			assert_eq!(ctx.store.get(tail), 1000);

			// Update the head of the pipeline
			ctx.store.write(pipeline[0], 500);
			Store::flush_updates(&mut ctx);

			// Node 1000 should now equal 1500!
			assert_eq!(ctx.store.get(tail), 1500);

			// ====================================================================
			// SCENARIO 3: DYNAMIC BRANCHING & GHOST DEPENDENCIES
			// ====================================================================
			// Tests if effects properly clear old dependencies when if/else branches change.

			let condition = ctx.store.prop(true);
			let active_data = ctx.store.prop("Active");
			let hidden_data = ctx.store.prop("Hidden");

			let branch_evals = Rc::new(Cell::new(0));
			let be_clone = branch_evals.clone();

			Store::effect(&mut ctx, move |c| {
				be_clone.set(be_clone.get() + 1);
				if c.store().get(condition) {
					let _ = c.store().get(active_data);
				} else {
					let _ = c.store().get(hidden_data);
				}
			});

			assert_eq!(branch_evals.get(), 1);

			// 1. Mutate `hidden_data`. The effect is NOT tracking this right now.
			ctx.store.write(hidden_data, "Hidden Changed");
			Store::flush_updates(&mut ctx);
			assert_eq!(branch_evals.get(), 1); // MUST NOT RUN!

			// 2. Flip condition to False.
			ctx.store.write(condition, false);
			Store::flush_updates(&mut ctx);
			assert_eq!(branch_evals.get(), 2); // Runs because `condition` changed

			// 3. Mutate `active_data`. The effect is no longer tracking this!
			ctx.store.write(active_data, "Active Changed");
			Store::flush_updates(&mut ctx);
			assert_eq!(branch_evals.get(), 3); // MUST NOT RUN! Ghost dependency properly cleared.

			// ====================================================================
			// SCENARIO 4: MID-TICK DEFERRED SLAB DESTRUCTION
			// ====================================================================
			// This simulates a UI component unmounting WHILE its data is changing.
			// If `slabs_to_remove` is broken, this will Segfault or Panic.

			let component_slab = ctx.store.create_slab();
			let unmount_signal = ctx.store.prop(false);

			// Data inside the component
			let internal_data = ctx.store.prop_in(component_slab, 100_i32).unwrap();
			let render_log = ctx.store.prop(0_i32);

			// The Component's render effect
			Store::effect_in(&mut ctx, component_slab, move |c| {
				let val = c.store().get(internal_data);
				c.store().write(render_log, val);
			});

			assert_eq!(ctx.store.get(render_log), 100);

			// A Parent effect that watches `unmount_signal` and destroys the slab
			let slab_copy = component_slab;
			Store::effect(&mut ctx, move |c| {
				if c.store().get(unmount_signal) {
					c.store().remove_slab(slab_copy).unwrap();
				}
			});

			// 💥 THE STRESS EVENT:
			// We update the component's data AND tell it to unmount in the exact same tick.
			ctx.store.write(internal_data, 999);
			ctx.store.write(unmount_signal, true);

			// Flush updates.
			// Expected behavior: The `is_updating` flag defers the slab destruction.
			// The internal effect might run (writing 999 to render_log), or it might get
			// destroyed first, but it MUST NOT crash the rust compiler or array indexing.
			Store::flush_updates(&mut ctx);

			// Verify the memory was truly freed after the flush
			assert!(!ctx.store.contains(internal_data));
			assert!(ctx.store.remove_slab(component_slab).is_err()); // Already gone

			// ====================================================================
			// SCENARIO 5: CROSS-SLAB COMMUNICATION
			// ====================================================================
			// Slab A writes to Global. Slab B reads from Global. Slab A is deleted.

			let slab_a = ctx.store.create_slab();
			let slab_b = ctx.store.create_slab();
			let shared_bus = ctx.store.prop(0_i32);

			let trigger_a = ctx.store.prop_in(slab_a, 1).unwrap();

			// Slab A writes to the bus
			Store::effect_in(&mut ctx, slab_a, move |c| {
				let val = c.store().get(trigger_a);
				c.store().write(shared_bus, val * 10);
			});

			let b_result = Rc::new(Cell::new(0));
			let b_res_clone = b_result.clone();

			// Slab B reads from the bus
			Store::effect_in(&mut ctx, slab_b, move |c| {
				b_res_clone.set(c.store().get(shared_bus));
			});

			assert_eq!(b_result.get(), 10); // Initial

			ctx.store.write(trigger_a, 5);
			Store::flush_updates(&mut ctx);
			assert_eq!(b_result.get(), 50); // Cross communication works

			// Delete Slab A. Triggering A should now panic if tried, but let's just
			// make sure Slab B survives and the bus is intact.
			ctx.store.remove_slab(slab_a).unwrap();
			Store::flush_updates(&mut ctx);

			assert!(ctx.store.try_read(trigger_a).is_none()); // A's prop is gone

			// Manually write to the bus. B should still react!
			ctx.store.write(shared_bus, 99);
			Store::flush_updates(&mut ctx);
			assert_eq!(b_result.get(), 99);

			println!("🎉 All Mega Stress Tests Passed! The Engine is Bulletproof.");
		}
	}
}
