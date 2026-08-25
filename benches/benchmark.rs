use criterion::{Criterion, criterion_group, criterion_main};
use smartpasslib::generate_smart_password_sync;

fn benchmark_generate_smart_password(c: &mut Criterion) {
    let secret = "MyStrongSecretPhrase2026!";
    c.bench_function("generate_smart_password_16", |b| {
        b.iter(|| {
            let _ = generate_smart_password_sync(secret, 16);
        })
    });
}

criterion_group!(benches, benchmark_generate_smart_password);
criterion_main!(benches);
