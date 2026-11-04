#!/usr/bin/env node
/**
 * fix-grammar.mjs
 *
 * Rewrites a gramex source tree with grammatical fixes applied to docs
 * and comments only. Code is never touched: every fix is an exact
 * doc/comment phrase match. Language, wording style and file format are
 * preserved; only the grammar is corrected.
 *
 * Usage:
 *   node fix-grammar.mjs <repoDir> <outDir>
 *
 * Example:
 *   node fix-grammar.mjs ./gramex ./gramex-fixed
 *
 * .rs / .md / .toml files get fixed, anything else is copied as is.
 * The directory layout is mirrored from input to output.
 */

import { readdirSync, readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join, dirname, extname, relative } from 'node:path';

const inputDir = process.argv[2] || '.';
const outputDir = process.argv[3] || './gramex-fixed';

// [search, replacement] pairs, applied as plain strings (no regex).
// Searches are exact doc/comment phrasing, so code lines never match.
const FIXES = [
	['bits matching get enabled', 'bits matching gets enabled'],
	['matching get enabled', 'matching gets enabled'],
	['it is created through [`b`] function', 'it is created through the [`b`] function'],
	['created by [`br`] function', 'created by the [`br`] function'],
	['in addition for the universal `&Bits`', 'in addition, for the universal `&Bits`'],
	['slice they matched.', 'slice they match.'],
	['panic if `len` is outside `1..=64`.', 'panics if `len` is outside `1..=64`.'],
	[
		'which implement [`MatchAble`] in little endian direction',
		'which implements [`MatchAble`] in the little-endian direction',
	],
	[
		'which implement [`MatchAble`] in big endian direction',
		'which implements [`MatchAble`] in the big-endian direction',
	],
	[
		'a bitfield of maximum 64 bits, able to be matched',
		'a bitfield of a maximum of 64 bits, able to be matched',
	],
	[
		', if `pred` returns `true` it matches with the field, else it fails.',
		'. If `pred` returns `true`, it matches with the field; otherwise, it fails.',
	],
	['[`Matcher`](crate::Matcher)s matches with', '[`Matcher`](crate::Matcher)s match with'],
	['[`Matcher`]s matches with', '[`Matcher`]s match with'],
	[
		'the `bytes` module export multiple [`Matcher`](crate::Matcher)s that matches and capture bytes',
		'the `bytes` module exports multiple [`Matcher`](crate::Matcher)s that match and capture bytes',
	],
	[
		'`bytes` is graduated to be [`BYTES`](Self::BYTES) sized',
		'`bytes` is guaranteed to be [`BYTES`](Self::BYTES) sized',
	],
	[
		'that convert `value` into little endian byte array',
		'that converts `value` into a little endian byte array',
	],
	[
		'that convert `value` into big endian byte array',
		'that converts `value` into a big endian byte array',
	],
	[
		'and test it by `pred`. if `pred` returns `true` it matches with the value, else it fails.',
		'and tests it against `pred`. If `pred` returns `true`, it matches with the value; otherwise, it fails.',
	],
	[
		'decodes a little endian byte slice as `T` value',
		'decodes a little endian byte slice as a `T` value',
	],
	[
		'decodes a big endian byte slice as `T` value',
		'decodes a big endian byte slice as a `T` value',
	],
	['# how it worksssss', '# how it works'],
	[
		'the generification of features though orthogonal set',
		'the generalization of features through an orthogonal set',
	],
	[
		'the `Mode` trait utilize GATs (generic associated types) and type state pattern to generic the matching logic over matching features',
		'the `Mode` trait utilizes GATs (generic associated types) and the type state pattern to make the matching logic generic over matching features',
	],
	[
		'this requirements are essential for ensuring flexible matching and zero copy parsing are universal property of matching.',
		'these requirements are essential for ensuring flexible matching and zero-copy parsing is a universal property of matching.',
	],
	[
		'can have variable length tokens, not necessarily be unitary sized.',
		'can have variable-length tokens and are not necessarily unitary in size.',
	],
	['for slice based types,', 'for slice-based types,'],
	[
		"`Matcher`s doesn't just return bool as the match result",
		"`Matcher`s don't just return `bool` as the match result",
	],
	[
		', take the [`MatchAble`] and an offset, and return a [`MatchResult`].',
		', takes the [`MatchAble`] and an offset, and returns a [`MatchResult`].',
	],
	[
		'the offset is a token aligned, can be forward if needed, must remain token aligned on success, but it can be anything on failure.',
		'the offset is token aligned, can be moved forward if needed, must remain token aligned on success, but can be anything on failure.',
	],
	[
		'for any [`MatchAble`] type, supports `T: Matcher`.',
		'for any [`MatchAble`] type, supporting `T: Matcher`.',
	],
	['else it acts as noop and always matches', 'otherwise it acts as a noop and always matches'],
	['the `Matcher` trait enable a type to behave', 'the `Matcher` trait enables a type to behave'],
	[
		'it monomorphize to the lightweightest matching logic, great for test based peeks',
		'it monomorphizes to the lightest matching logic, great for peek-based tests',
	],
	['peak based tests', 'peek-based tests'],
	[
		"great for grammar based extractions and data transformation that doesn't need",
		"great for grammar-based extractions and data transformations that don't need",
	],
	['returning `false` if there are excess input.', 'returning `false` if there is excess input.'],
	[
		'returning [excess](crate::result::MatchErrorKind::Excess) [`MatchError`] if there are excess input.',
		'returning an [excess](crate::result::MatchErrorKind::Excess) [`MatchError`] if there is excess input.',
	],
	[
		'returning `Err(())` if there are excess input.',
		'returning `Err(())` if there is excess input.',
	],
	['the `grammar` macro generate matchers', 'the `grammar` macro generates matchers'],
	['identified by a `name` that get transformed', 'identified by a `name` that gets transformed'],
	['generate an ananymous [`Matcher`]', 'generate an anonymous [`Matcher`]'],
	[
		'it return `bool` for if the expression matches',
		'it returns `bool` for whether the expression matches',
	],
	[
		'it return `Result<(), MatchError>` for if the expression matches',
		'it returns `Result<(), MatchError>` for whether the expression matches',
	],
	[
		'it return `Result<Capture, MatchError>` for if the expression matches',
		'it returns `Result<Capture, MatchError>` for whether the expression matches',
	],
	[
		'it return `Option<Capture>` for if the expression matches',
		'it returns `Option<Capture>` for whether the expression matches',
	],
	[
		'it return [excess](crate::result::MatchErrorKind::Excess) [`MatchError`] if there are excess input.',
		'it returns an [excess](crate::result::MatchErrorKind::Excess) [`MatchError`] if there is excess input.',
	],
	[
		'it return `false` if there are excess input.',
		'it returns `false` if there is excess input.',
	],
	['it return `None` if there are excess input.', 'it returns `None` if there is excess input.'],
	[
		'cursor module provide a solution for cases where',
		'cursor module provides a solution for cases where',
	],
	['non basic error side', 'non-basic error side'],
	[
		'context sensitive parsing, fully visible matching logic for deterministic and efficiency.',
		'context-sensitive parsing, fully visible matching logic for determinism and efficiency.',
	],
	[
		'tuple that act as a lightweight matching context',
		'tuple that acts as a lightweight matching context',
	],
	[
		'the [`Cursor`] provide the typical unconstrained imperative experience',
		'the [`Cursor`] provides the typical unconstrained imperative experience',
	],
	[
		'it hide all the manual offset handling and expose only',
		'it hides all the manual offset handling and exposes only',
	],
	['with cursor own twist', "with the cursor's own twist"],
	['custom enum based token list, which can be', 'custom enum-based token lists, which can be'],
	[
		'tuple that provide an imperative matching experience',
		'tuple that provides an imperative matching experience',
	],
	['// support mutliple errors', '// support multiple errors'],
	[
		'it fail atomically, and return `None` on failure.',
		'it fails atomically, and returns `None` on failure.',
	],
	['[`Mode`], and return `false` on failure.', '[`Mode`], and returns `false` on failure.'],
	[
		'under the hood, and act as noop at end of input.',
		'under the hood, and acts as a noop at end of input.',
	],
	[
		'depending if the `Cursor` is at the end of the input, then it get mapped',
		'depending on whether the `Cursor` is at the end of the input, then it gets mapped',
	],
	['generification for mathcing by token', 'generalization for matching by token'],
	['generification for mathcing by slice', 'generalization for matching by slice'],
	[
		'derive [`MatchAble`] for slice powered types.',
		'derive [`MatchAble`] for slice-powered types.',
	],
	[
		'`derive_slice_matchable` generate a [`MatchAble`] implementation',
		'`derive_slice_matchable` generates a [`MatchAble`] implementation',
	],
	[
		'applied on enums that for each variant define a [`Matcher`](crate::Matcher)',
		'applied on enums that for each variant defines a [`Matcher`](crate::Matcher)',
	],
	[
		'it take the [`MatchAble`] type, and define the [`Matcher`](crate::Matcher)',
		'it takes the [`MatchAble`] type, and defines the [`Matcher`](crate::Matcher)',
	],
	[
		'if the variant is tuple, the [`Capture`](crate::Matcher::Capture)',
		'if the variant is a tuple, the [`Capture`](crate::Matcher::Capture)',
	],
	['slice indexes as the offsets', 'slice indices as the offsets'],
	[
		'returns a new [`Matcher`] that redirect matching to the input, and rewind to start offset on failure.',
		'returns a new [`Matcher`] that redirects matching to the input, and rewinds to the start offset on failure.',
	],
	[
		'that capture the current [token](MatchAble::Token) and test it by `pred`. if `pred` returns `true` it matches with the token, else it fails.',
		'that captures the current [token](MatchAble::Token) and tests it against `pred`. If `pred` returns `true`, it matches with the token; otherwise, it fails.',
	],
	[
		'that capture a [slice](MatchAble::Slice) of `n` [tokens](MatchAble::Token) and test it by `pred`. if `pred` returns `true` it matches with the slice, else it fails.',
		'that captures a [slice](MatchAble::Slice) of `n` [tokens](MatchAble::Token) and tests it against `pred`. If `pred` returns `true`, it matches with the slice; otherwise, it fails.',
	],
	[
		'`expected` produces a [`Matcher`] that always fail with',
		'`expected` produces a [`Matcher`] that always fails with',
	],
	[
		'`fail_with` produces a [`Matcher`] that always fail with a [Other]',
		'`fail_with` produces a [`Matcher`] that always fails with an [Other]',
	],
	['`list` take an `item` and `sep`', '`list` takes an `item` and `sep`'],
	[
		'`delim_list` take a `start`, `item`, `sep` and `end`',
		'`delim_list` takes a `start`, `item`, `sep` and `end`',
	],
	['matches a maybe empty list of `item`s', 'matches a maybe-empty list of `item`s'],
	['closures must specify their arguments types', 'closures must specify their argument types'],
	[
		'wrapped functions dont get [`Mode`] generifications treatment, for that it is advised',
		"wrapped functions don't get the [`Mode`] generalization treatment; for that, it is advised",
	],
	[
		'the grammar expressions is a matching construct / DSL inspired by the typical metasyntax languages with a rust flavor.',
		'the grammar expressions are matching constructs / a DSL inspired by typical metasyntax languages with a rust flavor.',
	],
	['and mostly fail due to incomplete input.', 'and mostly fails due to incomplete input.'],
	['under the hode, it calls', 'under the hood, it calls'],
	['the bounds that can be of any atom type', 'the bounds can be of any atom type'],
	['transformed into ananymous [`Matcher`]s', 'transformed into anonymous [`Matcher`]s'],
	[
		'the arguments are matcher declerations with a root expression',
		'the arguments are matcher declarations with a root expression',
	],
	['enclosed by paranthesis.', 'enclosed by parentheses.'],
	[
		'the not operator also fail at incomplete input.',
		'the not operator also fails at incomplete input.',
	],
	[
		'for inverted result, incomplete input are positive.',
		'for inverted results, incomplete input is positive.',
	],
	[
		'and bounded one stop instantly when hitting max.',
		'and a bounded one stops instantly when hitting max.',
	],
	['and its negated form, it take precedence.', 'and its negated form, it takes precedence.'],
	[
		'the sequence expression match multiple expressions separated by whitespaces in order.',
		'the sequence expression matches multiple expressions separated by whitespace in order.',
	],
	[
		'with the same start and the first match win.',
		'with the same start and the first match wins.',
	],
	[
		'till where the first end, then it match the rest on it in order',
		'till where the first ends, then it matches the rest on it in order',
	],
	[
		'the imply expression express implication logical relation',
		'the imply expression expresses a logical implication',
	],
	[
		'it match the then expression after it, else it match nothing.',
		'it matches the then expression after it, otherwise it matches nothing.',
	],
	['there are 2 syntax for captures', 'there are 2 syntaxes for captures'],
	[
		'non optional specified [repetition](#repetitions) units, arguments of non captured [call atoms](#call-atom), and non capturing macros',
		'non-optional specified [repetition](#repetitions) units, arguments of non-captured [call atoms](#call-atom), and non-capturing macros',
	],
	['either by infering it from the optionality', 'either by inferring it from the optionality'],
	[
		'repetition override the capture resolved type, it become [`Option<T>`]',
		'repetition overrides the capture resolved type, it becomes [`Option<T>`]',
	],
	[
		'(without the [repetition](#repetition)) when requried.',
		'(without the [repetition](#repetition)) when required.',
	],
	['it is specified though `=> expr`', 'it is specified through `=> expr`'],
	[
		'the matched section is binded as the capture name.',
		'the matched section is bound as the capture name.',
	],
	['also they apply [`Into::into`]', 'they also apply [`Into::into`]'],
	['wihtout a [map](#mapping)', 'without a [map](#mapping)'],
	[
		'and [`Into::into`] convertion, and may need to specify their [type](#type-specifier) unless they are [call atom](#call-atom) to a local',
		'and [`Into::into`] conversion, and may need to specify their [type](#type-specifier) unless they are a [call atom](#call-atom) to a local',
	],
	[
		'by default they resolve to tuple of the nested captures captured values',
		"by default they resolve to a tuple of the nested captures' captured values",
	],
	[
		'the nested captures values are binded as their names, and the capture type become the map result.',
		"the nested captures' values are bound as their names, and the capture type becomes the map result.",
	],
	[
		'enum where each or branch resolve to a specific variant.',
		'enum where each or branch resolves to a specific variant.',
	],
	['in grammar decleration,', 'in grammar declarations,'],
	['outside grammar declerations', 'outside grammar declarations'],
	[
		'can generate there own items, structs and enums respectifilly.',
		'can generate their own items, structs and enums respectively.',
	],
	[
		"the generated items are public, has a `'src` lifetime, derive `Debug`",
		"the generated items are public, have a `'src` lifetime, derive `Debug`",
	],
	[
		"can generate a struct type, which become `Struct<'src>(MatchAble::Slice)`.",
		"can generate a struct type, which becomes `Struct<'src>(MatchAble::Slice)`.",
	],
	[
		'encode the kind of [`MatchError`], wheather it is',
		'encodes the kind of [`MatchError`], whether it is',
	],
	[
		'`MatchErrorKind` are generally not manually constructed, instead a contructor function',
		'`MatchErrorKind` is generally not manually constructed; instead, a constructor function',
	],
	[
		'a data structure that express what the matching logic',
		'a data structure that expresses what the matching logic',
	],
	['/// undifined `Expected` value.', '/// undefined `Expected` value.'],
	['/// expected whatever non empty thing.', '/// expected whatever non-empty thing.'],
	['/// usefull for [`skip_n`]', '/// useful for [`skip_n`]'],
	['being olny any string.', 'being only any string.'],
	['while [`Expected`] someething.', 'while [`Expected`] something.'],
	['/// the what part of [`MatchError`]', '/// the what of [`MatchError`]'],
	['where it occured,', 'where it occurred,'],
	['matches a character satisfing ', 'matches a character satisfying '],
	[', and capture it as `str` slice.', ', and captures it as a `str` slice.'],
	['`digit` create a [`Matcher`] that matches', '`digit` creates a [`Matcher`] that matches'],
	['offsets are byte indexes of [`str`].', 'offsets are byte indices of [`str`].'],
	[
		'it also adhere to rust zero cost abstraction principle, it leverage the power of [GATs](Mode) to enable its [`Matcher`]s get monomorphized',
		"it also adheres to rust's zero-cost abstraction principle, leveraging the power of [GATs](Mode) to enable its [`Matcher`]s to get monomorphized",
	],
	[
		'it also utilize zero copy parsing, it uses and produces',
		'it also utilizes zero-copy parsing, using and producing',
	],
	[
		'gramex feature its own custom [DSL](gram_ref), inspired by the typical metasyntax language, it has rich semantics. including and not limited to:',
		'gramex features its own custom [DSL](gram_ref); inspired by the typical metasyntax language, it has rich semantics, including but not limited to:',
	],
	[
		'this grammar expressions support powerfull [capturing abilities]',
		'this grammar expression supports powerful [capturing abilities]',
	],
	[
		'this expressions can be used everywhere, declared inside',
		'these expressions can be used everywhere: declared inside',
	],
	['or used [inline](parse!) in the normal code', 'or used [inline](parse!) in normal code'],
	['it can empowers [advanced parsers]', 'it can empower [advanced parsers]'],
	['its featureset can be fainly selected', 'its feature set can be finely selected'],
	[
		'will generate a tone of code that is highly optimizable by rustc, but is still enourmess',
		'will generate a ton of code that is highly optimizable by rustc, but is still enormous',
	],
	[
		'specilization of `Expr::Imply` in `Expr::Or`',
		'specialization of `Expr::Imply` in `Expr::Or`',
	],
	['use enum capture inspired approuch', 'use an enum-capture-inspired approach'],
	[
		'wihtout this the matcher const would overlap',
		'without this the matcher const would overlap',
	],
	['// mybe ok nested captures but errored self', '// maybe ok nested captures but errored self'],
	['expressions are naivly consumed', 'expressions are naively consumed'],
	[
		'for simplicity reasons, this work most of the time',
		'for simplicity reasons, this works most of the time',
	],
	[
		'though top level generic path segment and closure arguments must be enclosed inside paranthesis.',
		'though top-level generic path segments and closure arguments must be enclosed inside parentheses.',
	],
	['transform closure into `MathFn`', 'transform closure into `MatchFn`'],
	[
		'constructed through the varoius constructors',
		'constructed through the various constructors',
	],
	[
		'the `str` module export multiple [`Matcher`]s that matches characters',
		'the `str` module exports multiple [`Matcher`]s that match characters',
	],
	['generate ananymous `Matcher`', 'generate anonymous `Matcher`'],
	['that get binded to the expression', 'that get bound to the expression'],
	[
		'with [captures](crate::gram_ref#captures) binded by their name',
		'with [captures](crate::gram_ref#captures) bound by their name',
	],
	['/// else it evaluate the `else` arm', '/// otherwise it evaluates the `else` arm'],
	[
		'// `=> expr` mapping of the matched section (binded as `nb`)',
		'// `=> expr` mapping of the matched section (bound as `nb`)',
	],
	['where the error occured', 'where the error occurred'],
	['/// it take an offset where the error', '/// it takes an offset where the error'],
];

const FIXABLE = new Set(['.rs', '.md', '.toml']);
const SKIP_DIRS = new Set(['target', '.git', 'node_modules']);

let filesWritten = 0;
let filesChanged = 0;
let totalReplacements = 0;

function applyFixes(text) {
	let out = text;
	let count = 0;
	for (const [from, to] of FIXES) {
		if (!from) continue;
		const hits = out.split(from).length - 1;
		if (hits > 0) {
			out = out.split(from).join(to);
			count += hits;
		}
	}
	return [out, count];
}

function walk(dir) {
	const files = [];
	for (const entry of readdirSync(dir, { withFileTypes: true })) {
		const full = join(dir, entry.name);
		if (entry.isDirectory()) {
			if (!SKIP_DIRS.has(entry.name)) files.push(...walk(full));
		} else if (entry.isFile()) {
			files.push(full);
		}
	}
	return files;
}

function processFile(src) {
	const rel = relative(inputDir, src);
	const dest = join(outputDir, rel);
	mkdirSync(dirname(dest), { recursive: true });
	const text = readFileSync(src, 'utf8');

	if (!FIXABLE.has(extname(src))) {
		writeFileSync(dest, text);
		filesWritten++;
		return;
	}

	const [fixed, count] = applyFixes(text);
	writeFileSync(dest, fixed);
	filesWritten++;
	totalReplacements += count;
	if (count > 0) {
		filesChanged++;
		console.log(rel + ': ' + count + ' fix(es)');
	}
}

for (const file of walk(inputDir)) processFile(file);

console.log(
	'\nDone: ' +
		filesWritten +
		' files written to ' +
		outputDir +
		' (' +
		filesChanged +
		' files changed, ' +
		totalReplacements +
		' replacements).',
);
