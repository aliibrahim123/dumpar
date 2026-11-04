use gramex::{
	cursor::{Cursor, SimpleCursor, eat, match_map},
	derive::derive_enum_matcher,
	derive_slice_matchable,
	general::list,
	grammar, matches, parse,
	result::MatchError,
	str::dec,
};

fn basics() {
	// `matches` return `true` if a value matches a pattern
	// patterns are separated by whitespace, and can be literals, paths and blocks
	let c = "c";
	assert!(matches!("abc", 'a' "b" c));

	// `~` matches without advancing, `!` matches one token if its pattern fails
	assert!(matches!("ac", ~"ac" !"b" 'c'));

	// '?': optional, '+': 1..inf repetition, `[n]`: exact repetition
	assert!(matches!("bbccc", "a"? 'b'+ 'c'[3]));

	// `..`: range matching, `_`: matches one token
	assert!(matches!("abc", 'a'..'z' _ ));

	// `|`: match any pattern, `&`: match all patterns
	assert!(matches!("a", 'a' | 'b' | 'c'));
	assert!(matches!("b", 'a'..'z' & !'c'));

	// captures: extract the matched section
	assert!(parse!("abc", 'a' bc:"bc").is_ok_and(|(bc,)| bc == "bc"));
}

// grammer declaration, define multiple matchers
grammar! {
	for str;
	let ident: String = ('a'..'z' | 'A'..'Z' | '0'..'9' | '_')+;
	// `=> expr` mapping of the matched section (bound as `nb`)
	let nb: i64 = '-'? ('0'..'9')+ => nb.parse().unwrap();
	// `path<args>` compound matchers, `list<item, sep>`: list of `sep` seprated `item`s
	let arr: Vec<Val<'src>> = '[' list:list<val, ','> ']' => list;
	// generate an enum `Val`, can also use predefined matchers
	let val: enum = true:"true" | false:"false" | ident:ident | nb:nb | arr:arr;
}

// (input, offset) tuple with some utilities, for ergonomic advance cases
type Cur<'src> = SimpleCursor<'src, str>;
fn parse_primary(cur: &mut Cur) -> Result<Expr, MatchError> {
	// `try_eat`: optional match by matcher, even from grammer declarations
	if let Some(num) = cur.try_eat(nb) {
		Ok(Expr::Num(num))
	} else if let Some(_ident) = cur.try_eat(ident) {
		Ok(Expr::Ident(_ident))
	} else {
		// become `expected one of identifier, number`
		cur.expected(["identifier", "number"])
	}
}
fn parse_expr(cur: &mut Cur) -> Result<Expr, MatchError> {
	// `eat`: match by a grammer expression
	let (name,) = eat!(cur, name:ident '=')?;
	let mut expr = parse_primary(cur)?;
	'op_loop: loop {
		// `match_map`: gramex `match` expression
		match_map!(cur, {
			'+' => expr = Expr::Add(Box::new(expr), Box::new(parse_primary(cur)?)),
			'-' => expr = Expr::Sub(Box::new(expr), Box::new(parse_primary(cur)?)),
			else => break 'op_loop,
		})
	}
	cur.eat(';')?;
	Ok(Expr::Let(name, Box::new(expr)))
}

enum Expr {
	Ident(String),
	Num(i64),
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Let(String, Box<Expr>),
}
