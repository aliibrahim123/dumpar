use neoview_web::prelude::{DomContext, StoreProv};

fn a(ctx: &mut DomContext) {
	let store = ctx.store();
	let a = store.prop(1);
	let b = store.prop(String::from("abc"));
	let c = store.prop(true);

	// you can reference multiple properties tagather
	println!("{} {} {}", store.read(a), store.read(b), store.read(c));

	// you can write exactly one property while there are no active reference
	store.write(a, 2);

	// dont worry and enfore the read compute write pattern
	store.write(b, format!("{}: {}", store.read(b), store.read(a)));

	// you also have convinence methods
	let _: u64 = store.get(a);
	*store.read_mut(a) += 1;
	store.update(c, |v| *v ^= true);

	// this pattern covers 99.99% of the cases
	// however if you realy nead to mutate multiple properties at the same time
	let (a_value, b_value) = store.read_disjoint_mut((a, b));
	*a_value += 1;
	b_value.push_str("def");
}
