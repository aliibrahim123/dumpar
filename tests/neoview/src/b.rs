use std::{
	cell::{Cell, RefCell},
	collections::HashSet,
	mem::forget,
	rc::Rc,
};

use js_sys::{Date, Promise, Undefined, futures::spawn_local};
use neoview::{Error, Store};
use neoview_web::{
	CtxHandle,
	apply::{
		self, Applicable, id, show_if, style,
		tags::{br, button, div, span},
		text,
	},
	new_ctx,
	prelude::*,
	render_list_enumerated, use_ctx,
};
use wasm_bindgen::prelude::{Closure, JsCast, JsValue, ScopedClosure, wasm_bindgen};
use web_sys::{
	HtmlElement, HtmlInputElement, ScrollIntoViewOptions,
	console::{self, time_end_with_label, time_with_label},
	window,
};

macro_rules! println {
	($($t:tt)*) => {
		console::log_1(&format_args!($($t)*).to_string().into())
	};
}

pub mod rand {
	use super::*;
	use std::ops::Range;

	use js_sys::Math::random;

	pub fn random_range(range: Range<usize>) -> usize {
		let v = (random() * (range.end - range.start) as f64 + range.start as f64) as usize;
		//println!("{v}");
		v
	}
}

fn focus(prop: PropId<bool>) -> impl Applicable {
	move |build: &mut ChunkBuild| {
		build.ref_el(move |ctx, el| {
			let el = el.clone().dyn_into::<HtmlElement>().unwrap();
			ctx.effect(move |ctx| {
				if ctx.get(prop) {
					el.focus().unwrap()
				}
			});
		});
	}
}

#[wasm_bindgen(start)]
pub fn main_js() {
	// Call this once at the start of your app

	time_with_label("rendering");
	console_error_panic_hook::set_once();
	let document = window().unwrap().document().unwrap();

	let handle = new_ctx(document.query_selector("#main").unwrap().unwrap(), Default::default());
	let mut ctx = handle.borrow().unwrap();
	let ctx_id = ctx.id();
	let mut build = ctx.root_chunk();

	let id = "input";
	let input = build.prop("123".to_string());
	let html = build.computed(move |ctx| JsValue::from(ctx.read(input)));
	let count1 = build.prop(0u64);
	let count2 = build.prop(0u64);
	build.effect(move |ctx| {
		ctx.write(count1, ctx.get(count2));
	});
	let list = build
		.computed(move |ctx| ctx.read(input).split(",").map(|v| v.to_owned()).collect::<Vec<_>>());
	let (section, remove) = {
		let mut build = build.ctx().removable_chunk("div");
		chunk!(&mut build, "hallo all", input, move |ctx| {
			let el = window().unwrap().document().unwrap().create_element("div").unwrap();
			el.set_inner_html(&ctx.read(input));
			el
		});
		build.effect(move |ctx| println!("{}", ctx.read(input)));
		#[rustfmt::skip]
		build.apply(apply::render_list_enumerated(list, |v| v.clone(), "div", |mut builda, item, i| {
			builda.effect(move |ctx| println!("id: {}", ctx.get(i)));
			chunk!(builda, item);
		}));
		build.build()
	};
	let remove = Cell::new(Some(remove));

	let a = 1;
	let b = 2;
	let nb = 3;
	{
		let store = build.store();
		let a = store.prop(1);
		let slab = store.create_slab();
		let b = store.prop_in(Some(slab), 2).unwrap();
		let c = store.prop(3);

		let (a_val, b_val, c_val) = store.read_disjoint_mut((a, b, c));
		*a_val += 1;
		*b_val += 1;
		*c_val += 1;

		assert_eq!(store.get(a), 2);
		assert_eq!(store.get(b), 3);
		assert_eq!(store.get(c), 4);
		assert_eq!(store.try_read_disjoint_mut((a, b, c)), Ok((&mut 2, &mut 3, &mut 4)));
		Store::remove_slab(build.ctx(), slab);
		let store = build.store();
		assert_eq!(store.try_read_disjoint_mut((a, b, c)), Err(Error::Removed));
		assert_eq!(store.try_read_disjoint_mut((a, c, a)), Err(Error::NotDisjoint));
	}

	chunk!(build, div {
		style { ".red { color: red }" }
		input(value: &&123, on.input: (move |ctx, evt| {
			let el = evt.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
			ctx.write(input, el.value());
		}), id, aria_description: "123")
		button (
			on.click:(move |ctx, _| {ctx.update(count2, |v| *v +=1);}),
			style.background: move |ctx| if ctx.get(count2) % 4 < 2 { Some("red") } else {None}
		) {
			move |ctx| format!("count: {}", ctx.get(count2))
		}
		p { "hallo ", move |ctx| ctx.get(count1) * 10 }
		span (class.red: move |ctx| ctx.get(count1) % 20 < 10) { "text: ",
			span (prop.innerHTML: html)
		}
		div(data-attr: "a", class.my-div: true, style."color": "blue") {
			button (on.click: (move |ctx, _| match remove.replace(None) {
				Some(remove) => remove.remove(ctx),
				None => ()
			}), apply: show_if(move |ctx: &mut DomContext| ctx.get(count1) % 200 < 100)) {"delete"}
			section
		}
		do {
			let count = build.computed(move |ctx| ctx.get(count2) * 100);
			if Date::now() % 10000. < 500. {
				build.apply(div(text("hallo")));
			}
			for i in 0..10 {
				build.apply((
					text(format!("count {i}: ")),
					text(move |ctx: &mut DomContext| ctx.get(count2) * i),
					br(())
				));
			}
			build.ref_el(move |_, el| el.before_with_str_1("ali").unwrap());
			chunk!(build, div {
				"hallo x", count,
				do { build.apply(style("background", "orange")) },
				do { neoview_web::show_if(build, move |ctx: &mut DomContext| ctx.get(count) > 500) }
			})
		}
		div{do {
			render_list_enumerated(&mut build, list, |v| v.to_string(), "div",
				move |mut build, v, i| chunk!(build, i, ": ", v)
			)
		}}
		div { do {
			build.apply(show_if(false));
			let el = Rc::new(RefCell::new(None));
			let el_ref = Rc::clone(&el);
			build.ref_el(move |_, el| *el_ref.borrow_mut() = Some(el.clone()));
			let list = build.prop((0..1000).collect::<Vec<_>>());
			render_list_enumerated(&mut build, list, |v| *v, "div",
				move |mut builda, v, i| chunk!(builda, i, ": ", v, ",")
			);
			handle.use_ctx(move |ctx| for i in 0..2 {
				let old_list = ctx.read(list).clone();
				println!("{i}");
				let el = el.borrow().as_ref().unwrap().clone();
				ctx.update(list, |list| {
					for _ in 0..rand::random_range(0..300) {
						let len = list.len();
						match rand::random_range(0..3) {
							0 => list.insert(rand::random_range(0..len), rand::random_range(0..1000)),
							1 if len > 0 => {list.remove(rand::random_range(0..len));},
							2 if len > 0 => list.swap(rand::random_range(0..len), rand::random_range(0..len)),
							_ => ()
						};
					}
				});
				Store::flush_updates(ctx);
				let list = ctx.read(list);
				//println!("{old_list:?} -> {list:?}, redered: {}", el.inner_html());
				let children = el.children();
				assert_eq!(children.length() as usize, list.len());
				for i in 0..children.length() {
					assert_eq!(
						children.item(i).unwrap().dyn_into::<HtmlElement>().unwrap().text_content().unwrap(),
						format!("{i}: {},", list[i as usize])
					)
				}
			});
			//interval(move || use_ctx(ctx_id, |ctx| ctx.update(list, |v| v.rotate_left(2))).unwrap(), 150);
		} }

		"after this",
		do {
			let count = build.prop(0);
			chunk!(build, button(
				on.click: (move |ctx, _| ctx.update(count, |v| *v += 1))
			) { "count: ", count });
		}
		{"before this"},
	});

	let date = Date::new_0();
	struct A {
		a: u64,
		b: u64,
	};
	let a_list = vec![A { a: 1, b: 2 }, A { a: 3, b: 4 }, A { a: 5, b: 6 }];
	chunk!(build, div {
		if date.get_seconds() % 2 == 0 {
			"hallo"
		}
		else if date.get_seconds() % 3 == 0 {
			"hallo2"
		}
		else {
			div (style.color:"red") { "try later" }
		}
		if true { " ", "always" }
		for i in 0..date.get_seconds() % 10 {
			"item ", i, br()
		}
		for A { a, b } in a_list {
			"a: ", a, ", b: ", b, "  "
		}
		match date.get_seconds() % 3 {
			0 => { "a" }
			1 => span { "b" },
			_ => "c"
		}
	});

	{
		use neoview_web::apply::*;
		let mut root_build = &mut build;
		let mut build = root_build.ctx().new_chunk_tagged("div");

		let count = build.prop(0);
		chunk!(build, div(id: "section", style.color: "red") {
			"hello world",
			for i in 0..10 {
				"item ", i, br()
			}
			button(on.click: (move |ctx, _| ctx.update(count, |v| *v += 1))) { "count: ", count }
		});

		build.apply(div((
			id("section"),
			style("color", "red"),
			text("hello world"),
			move |build: &mut ChunkBuild| {
				for i in 0..10 {
					build.apply((text(format!("item {i}")), br(())));
				}
			},
			button((
				on("click", move |ctx, _| ctx.update(count, |v| *v += 1)),
				text("count: "),
				text(count),
			)),
		)));

		let el = build.build();
		chunk!(root_build, el);
	}

	{
		let mut root_build = &mut build;
		let mut build = root_build.ctx().new_chunk_tagged("div");
		chunk!(build, "hello world");
		let el = build.build();
		chunk!(root_build, el);

		let mut build = root_build.ctx().removable_chunk("div");
		chunk!(build, "hello world");
		let (el, remover) = build.build();
		let mut remover = Some(remover);
		chunk!(root_build, el,
			button(on.click: (move |ctx, _| remover.take().unwrap().remove(ctx))) { "remove" }
		);
	}

	{
		let mut build = build.ctx().new_chunk_tagged("div");
		chunk!(build, "starting");
		let (ctx, state) = build.hibernate();
		ctx.write(count2, 2);
		let id = ctx.id();
		let fun = move |ctx: &mut DomContext| {
			let mut build = state.wake(ctx);
			chunk!(build, br() "waked");
			let el = build.build();
			let mut build = ctx.root_chunk();
			chunk!(build, el);
			build.build();
		};
		set_timeout(move || use_ctx(id, fun).unwrap(), 1000);
	}
	build.build();

	{
		let handle = ctx.new_handle();
		let fun = move || {
			let handle = handle.borrow().unwrap().new_handle();
			time_with_label("chaos 1");
			for _ in 0..1000 {
				handle.use_ctx(move |ctx| ctx.update(count2, |v| *v += 1))
			}
			time_end_with_label("chaos 1");
			set_timeout(move || handle.borrow().unwrap().update(count2, |v| *v = 1), 100);
		};
		set_timeout(fun, 1000);
	}

	println!("{}", document.body().unwrap().inner_html());

	let handle2 = handle.clone();
	spawn_local(async move {
		for i in 0..10 {
			let mut ctx = handle2.acquire().await;
			let mut build = ctx.root_chunk();
			chunk!(build, "content ", i, br());
			build.build();
			drop(ctx);
			delay(1000).await
		}
	});

	drop(ctx);
	forget(handle);
	for _ in 0..100 {
		let handle = new_ctx(document.create_element("a").unwrap(), Default::default());
		let _ctx = handle.borrow();
		let id = handle.id();
		handle.use_ctx(|_| println!("a"));
		set_timeout(move || use_ctx(id, |_| println!("b")).unwrap_or(()), 100);
	}
	interval(move || use_ctx(ctx_id, move |ctx| ctx.update(count1, |v| *v += 1)).unwrap(), 100);

	time_end_with_label("rendering");
	println!("finished rust");
}

fn interval(fun: impl Fn() + 'static, delay: i32) {
	let closure = Closure::<dyn Fn()>::new(fun);
	window()
		.unwrap()
		.set_interval_with_callback_and_timeout_and_arguments_0(
			closure.as_ref().unchecked_ref(),
			delay,
		)
		.unwrap();
	closure.forget();
}
fn set_timeout(fun: impl FnOnce() + 'static, delay: i32) {
	let mut fun = Some(fun);
	let closure = Closure::<dyn FnMut()>::new(move || fun.take().unwrap()());
	window()
		.unwrap()
		.set_timeout_with_callback_and_timeout_and_arguments_0(
			closure.as_ref().unchecked_ref(),
			delay,
		)
		.unwrap();
	closure.forget();
}

async fn delay(delay: i32) {
	Promise::new(&mut |resolve, _| {
		set_timeout(move || resolve.call0(&0.into()).__(), delay);
	})
	.await;
}

trait Noop {
	fn __(&self) {}
}
impl<T> Noop for T {}
