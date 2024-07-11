use criterion::{criterion_group, criterion_main, Criterion};

const GLOB_NORMAL: &'static str = "some/**/n*d[k-m]e?txt";
const GLOB_BRACES: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

fn glob(b: &mut Criterion) {
  // glob #1
  b.bench_function("benchmark", |b| {
    b.iter(|| assert!(glob::Pattern::new(GLOB_NORMAL).unwrap().matches(PATH)))
  });
}

fn mine(b: &mut Criterion) {
  // mine #2
  b.bench_function("benchmark", |b| {
    b.iter(|| assert!(fast_glob::glob_match(GLOB_NORMAL, PATH)))
  });
}

fn globset(b: &mut Criterion) {
  // globset #3
  b.bench_function("benchmark", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB_NORMAL)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });
}

fn globmatch(b: &mut Criterion) {
  // glob_match #4
  b.bench_function("benchmark", |b| {
    b.iter(|| assert!(glob_match::glob_match(GLOB_NORMAL, PATH)))
  });
}

fn mine_braces(b: &mut Criterion) {
  // mine #5
  b.bench_function("benchmark", |b| {
    b.iter(|| assert!(fast_glob::glob_match_with_brace(GLOB_BRACES, PATH)));
  });
}

fn globset_braces(b: &mut Criterion) {
  // globset #6
  b.bench_function("benchmark", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB_BRACES)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });
}

fn globmatch_braces(b: &mut Criterion) {
  // glob_match #7
  b.bench_function("benchmark", |b| {
    b.iter(|| assert!(glob_match::glob_match(GLOB_BRACES, PATH)))
  });
}

criterion_group!(
  benches,
  glob,
  mine,
  globset,
  globmatch,
  mine_braces,
  globset_braces,
  globmatch_braces
);
criterion_main!(benches);
