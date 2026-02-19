use criterion::{Criterion, criterion_group, criterion_main};
use gen_ser::{
    models::Person,
    serializer::Serializer,
    serializers::{Borsh, SerdeJson, Wincode},
};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let person = Person {
        name: "devwraithe".to_string(),
        age: 25,
    };

    // Serialization benchmarks
    let mut group = c.benchmark_group("Serialization");

    // Benchmark each serialization formats in one group
    group.bench_function("Borsh", |b| {
        let borsh = Borsh;
        b.iter(|| borsh.to_bytes(black_box(&person)).unwrap())
    });

    group.bench_function("Wincode", |b| {
        let wincode = Wincode;
        b.iter(|| wincode.to_bytes(black_box(&person)).unwrap())
    });

    group.bench_function("SerdeJson", |b| {
        let serde_json = SerdeJson;
        b.iter(|| serde_json.to_bytes(black_box(&person)).unwrap())
    });

    // Finish benchmark for this group and generate summary
    group.finish();

    // Deserialization benchmarks
    let borsh_bytes = Borsh.to_bytes(&person).unwrap();
    let serde_json_bytes = SerdeJson.to_bytes(&person).unwrap();
    let wincode_bytes = Wincode.to_bytes(&person).unwrap();

    let mut group = c.benchmark_group("Deserialization");

    // Benchmark each deserialization formats in one group
    group.bench_function("Borsh", |b| {
        let borsh = Borsh;
        let borsh_fb: Person = borsh.from_bytes(black_box(&borsh_bytes)).unwrap();
        b.iter(|| &borsh_fb)
    });

    group.bench_function("Wincode", |b| {
        let wincode = Wincode;
        let wincode_fb: Person = wincode.from_bytes(black_box(&wincode_bytes)).unwrap();
        b.iter(|| &wincode_fb)
    });

    group.bench_function("SerdeJson", |b| {
        let serde_json = SerdeJson;
        let serde_json_fb: Person = serde_json.from_bytes(black_box(&serde_json_bytes)).unwrap();
        b.iter(|| &serde_json_fb)
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
