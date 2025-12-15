use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prospector_core_math::private_key::SafePrivateKey;
use prospector_core_math::public_key::SafePublicKey;

fn benchmark_pubkey_generation(c: &mut Criterion) {
    let private_key = SafePrivateKey::new_random();

    c.bench_function("secp256k1_pubkey_gen", |b| {
        b.iter(|| {
            // Medimos cuánto tarda en derivar la PubKey (operación más costosa)
            // black_box evita que el compilador optimice y elimine el código
            let _pk = SafePublicKey::from_private(black_box(&private_key));
        })
    });
}

criterion_group!(benches, benchmark_pubkey_generation);
criterion_main!(benches);
