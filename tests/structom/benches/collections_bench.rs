use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::prelude::RngExt;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rustc_hash::FxHashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// 1. Sparse Vec: Index acts as the ID. High memory usage for sparse data, but O(1) ops.
#[derive(Clone)]
pub struct SparseVec<T> {
	data: Vec<Option<T>>,
}
impl<T: Clone> SparseVec<T> {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}
	pub fn insert(&mut self, id: usize, val: T) {
		if id >= self.data.len() {
			self.data.resize(id + 1, None);
		}
		self.data[id] = Some(val);
	}
	pub fn get(&self, id: usize) -> Option<&T> {
		self.data.get(id).and_then(|v| v.as_ref())
	}
}

/// 2. Sorted Dense Vec: Dense memory layout, requires O(log N) binary search for lookup/insert.
#[derive(Clone)]
pub struct SortedDenseVec<T> {
	data: Vec<(usize, T)>,
}
impl<T> SortedDenseVec<T> {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}
	pub fn insert(&mut self, id: usize, val: T) {
		match self.data.binary_search_by_key(&id, |&(i, _)| i) {
			Ok(idx) => self.data[idx] = (id, val),
			Err(idx) => self.data.insert(idx, (id, val)),
		}
	}
	pub fn get(&self, id: usize) -> Option<&T> {
		self.data
			.binary_search_by_key(&id, |&(i, _)| i)
			.ok()
			.map(|idx| &self.data[idx].1)
	}
}

/// 3. Indexed Vec: Fast FxHashMap maps ID -> Index, actual data is densely packed in a Vec.
#[derive(Clone)]
pub struct MapIndexedVec<T> {
	map: FxHashMap<usize, usize>,
	data: Vec<T>,
}
impl<T> MapIndexedVec<T> {
	pub fn new() -> Self {
		Self { map: FxHashMap::default(), data: Vec::new() }
	}
	pub fn insert(&mut self, id: usize, val: T) {
		if let Some(&idx) = self.map.get(&id) {
			self.data[idx] = val;
		} else {
			self.map.insert(id, self.data.len());
			self.data.push(val);
		}
	}
	pub fn get(&self, id: usize) -> Option<&T> {
		self.map.get(&id).map(|&idx| &self.data[idx])
	}
}

/// 4. Direct Map: Just using FxHashMap directly as a baseline comparison.
#[derive(Clone)]
pub struct DirectFxHashMap<T> {
	data: FxHashMap<usize, T>,
}
impl<T> DirectFxHashMap<T> {
	pub fn new() -> Self {
		Self { data: FxHashMap::default() }
	}
	pub fn insert(&mut self, id: usize, val: T) {
		self.data.insert(id, val);
	}
	pub fn get(&self, id: usize) -> Option<&T> {
		self.data.get(&id)
	}
}

// ============================================================================
// GENERATORS (Access Patterns)
// ============================================================================

fn gen_linear(size: usize) -> Vec<usize> {
	(0..size).collect()
}

fn gen_random(size: usize) -> Vec<usize> {
	let mut rng = StdRng::seed_from_u64(42);
	// Multiplying by 2 forces some sparsity into the random keys
	(0..size).map(|_| rng.random_range(0..size * 2)).collect()
}

fn gen_slightly_randomly_linear(size: usize) -> Vec<usize> {
	let mut ids = gen_linear(size);
	let mut rng = StdRng::seed_from_u64(42);
	for i in 0..size {
		if rng.random_bool(0.1) {
			// 10% chance to swap with a nearby element, making it slightly out of order
			let offset = rng.random_range(1..5);
			let swap_idx = (i + offset).min(size - 1);
			ids.swap(i, swap_idx);
		}
	}
	ids
}

// ============================================================================
// BENCHMARK MACROS & RUNNERS
// ============================================================================

macro_rules! bench_insert {
	($group:expr, $pattern_name:expr, $size:expr, $ids:expr, $Type:ident) => {
		$group.bench_with_input(
			BenchmarkId::new(format!("{}_{}", stringify!($Type), $pattern_name), $size),
			&$ids,
			|b, ids| {
				// iter_batched excludes Drop time and clean initialization time from the benchmark
				b.iter_batched(
					|| $Type::new(),
					|mut ds| {
						for &id in ids {
							ds.insert(id, 42u64); // Inserting dummy payload
						}
						ds
					},
					criterion::BatchSize::SmallInput,
				);
			},
		);
	};
}

macro_rules! bench_read {
	($group:expr, $pattern_name:expr, $size:expr, $ids:expr, $Type:ident) => {
		let mut ds = $Type::new();
		// Pre-populate data structure before timing reads
		for &id in &$ids {
			ds.insert(id, 42u64);
		}
		$group.bench_with_input(
			BenchmarkId::new(format!("{}_{}", stringify!($Type), $pattern_name), $size),
			&$ids,
			|b, ids| {
				b.iter(|| {
					for &id in ids {
						black_box(ds.get(id));
					}
				});
			},
		);
	};
}

fn bench_inserts(c: &mut Criterion) {
	let mut group = c.benchmark_group("Insertions");
	let sizes = [5, 15]; // Test different scaling constraints

	for &size in &sizes {
		let linear_ids = gen_linear(size);
		let random_ids = gen_random(size);
		let slight_ids = gen_slightly_randomly_linear(size);

		// Linear
		bench_insert!(group, "Linear", size, linear_ids, SparseVec);
		bench_insert!(group, "Linear", size, linear_ids, MapIndexedVec);

		// Slightly Random
		bench_insert!(group, "Slight_Random", size, slight_ids, SparseVec);
		bench_insert!(group, "Slight_Random", size, slight_ids, MapIndexedVec);
	}
	group.finish();
}

fn bench_reads(c: &mut Criterion) {
	let mut group = c.benchmark_group("Reads");
	let sizes = [5, 15];

	for &size in &sizes {
		let linear_ids = gen_linear(size);
		let random_ids = gen_random(size);
		let slight_ids = gen_slightly_randomly_linear(size);

		// Linear
		bench_read!(group, "Linear", size, linear_ids, SparseVec);
		bench_read!(group, "Linear", size, linear_ids, MapIndexedVec);

		// Slightly Random
		bench_read!(group, "Slight_Random", size, slight_ids, SparseVec);
		bench_read!(group, "Slight_Random", size, slight_ids, MapIndexedVec);
	}
	group.finish();
}

criterion_group!(benches, bench_inserts, bench_reads);
criterion_main!(benches);
