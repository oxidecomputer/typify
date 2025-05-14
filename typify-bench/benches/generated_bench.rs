use criterion::{criterion_group, criterion_main};

/// Import a benchmark test case in a private module, using schema types generated
/// by build.rs. The imported benchmark can be executed using {module_name}::bench.
/// 
/// ## Parameters
/// - name: the name of the module and the stem of the JSON schema source. The schema file must
///         be located in benches/{name}.json.
/// - closure: the closure containing the benchmark procedure, which should exercise types
///         from the source schema.
macro_rules! bench_module {
    ($name:ident, $closure:expr) => {
        mod $name {
            include!(concat!(env!("OUT_DIR"), "/", stringify!($name), ".rs"));

            pub fn bench(c: &mut criterion::Criterion) {
                c.bench_function(stringify!($name), |b| b.iter($closure));
            }
        }
    };
}

bench_module!(regex_long, || {
    let _result: TypifyValidationBenchmark = serde_json::from_str("{\"name\": \"hello_world\"}").unwrap();
});
bench_module!(regex_short, || {
    let _result: IdOrName = "hello-worlD".parse().unwrap();
});

criterion_group!(benches, regex_long::bench, regex_short::bench);
criterion_main!(benches);