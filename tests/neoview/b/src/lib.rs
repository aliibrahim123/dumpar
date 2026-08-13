use std::ops::Not;

use neoview_web::{apply::show_if, new_ctx, prelude::*, render_list};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main_js() {
	let el = window().unwrap().document().unwrap().query_selector("#main").unwrap().unwrap();
	let handle = new_ctx(el, Default::default());
	{
		let mut ctx = handle.borrow().unwrap();
		let mut build = ctx.root_chunk();
		chunk!(build, div {
			h3 { "Hello, world!" }
			do {
				let count = build.prop(0);
				chunk!(build, button(
					on.click: (move |ctx, _| ctx.update(count, |v| *v += 1))
				) { "count: ", count });

				fn counter(mut build: &mut ChunkBuild, name: &str) {
					 let count = build.prop(0);
					 chunk!(build, button(
						 on.click: (move |ctx, _| ctx.update(count, |v| *v += 1))
					 ) { name, ": ", count });
				 }
				 for i in 0..10 {
					 counter(&mut build, &format!("counter {i}"));
					 match i % 3 {
						 0 | 1 => chunk!(build, br()),
						 _ => chunk!(build, hr()),
					 }
				 }
				 let hidden = build.prop(true);
				 chunk!(build,
					 span(id: "an-element")
					 input(hidden)
					 a(href: move |ctx| ctx.get(hidden).not().then_some("https://example.com"))
				 );
			}
		});
		chunk!(build, div {
			do {
				let list = build.prop(vec![1, 2, 3]);
				build.apply(show_if(move |ctx: &mut DomContext| ctx.read(list).len() > 0));
				render_list(build, list, |v| *v, "div", |mut build, v| chunk!(build, v));
			}
		});
		build.build();
	}
	std::mem::forget(handle);
}
