use criterion::{criterion_group, criterion_main, Criterion};

const GLOB_NORMAL: &'static str = "some/**/needle.txt";
const GLOB_BRACES: &'static str = "some/**/{the,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

fn glob(b: &mut Criterion) {
  b.bench_function("glob", |b| {
    b.iter(|| assert!(glob::Pattern::new(GLOB_NORMAL).unwrap().matches(PATH)))
  });
}

fn mine(b: &mut Criterion) {
  b.bench_function("mine_normal", |b| {
    b.iter(|| assert!(fast_glob::glob_match(GLOB_NORMAL, PATH)))
  });
}

fn globset(b: &mut Criterion) {
  b.bench_function("globset_normal", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB_NORMAL)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });
}

fn mine_braces(b: &mut Criterion) {
  b.bench_function("mine_braces", |b| {
    b.iter(|| assert!(fast_glob::glob_match_with_brace(GLOB_BRACES, PATH)));
  });
}

fn globset_brace(b: &mut Criterion) {
  b.bench_function("globset_braces", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB_BRACES)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });
}

criterion_group!(benches, glob, mine, globset, mine_braces, globset_brace);
criterion_main!(benches);
