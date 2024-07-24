use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};

fn simple_match(c: &mut Criterion) {
  let mut group = c.benchmark_group("Simple Match");

  const GLOB: &'static str = "some/**/n*d[k-m]e?txt";
  const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

  group.bench_function("glob", |b| {
    b.iter(|| assert!(glob::Pattern::new(GLOB).unwrap().matches(PATH)))
  });

  group.bench_function("globset", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });

  group.bench_function("glob-match", |b| {
    b.iter(|| assert!(glob_match::glob_match(GLOB, PATH)))
  });

  group.bench_function("fast-glob", |b| {
    b.iter(|| assert!(fast_glob::glob_match(GLOB, PATH)))
  });

  group.finish();
}

fn brace_expansion(c: &mut Criterion) {
  let mut group = c.benchmark_group("Brace Expansion");

  const GLOB: &'static str = "some/**/{tob,crazy}/?*.{png,txt}";
  const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

  group.bench_function("globset", |b| {
    b.iter(|| {
      assert!(globset::Glob::new(GLOB)
        .unwrap()
        .compile_matcher()
        .is_match(PATH));
    })
  });

  group.bench_function("glob-match", |b| {
    b.iter(|| assert!(glob_match::glob_match(GLOB, PATH)))
  });

  group.bench_function("fast-glob", |b| {
    b.iter(|| assert!(fast_glob::glob_match_with_brace(GLOB, PATH)));
  });

  group.finish();
}

criterion_group!(benches, simple_match, brace_expansion);
criterion_main!(benches);
