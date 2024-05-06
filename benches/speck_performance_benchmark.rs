use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use hex_literal::hex;
use speck_cipher::{speck_cbc_decrypt, speck_cbc_encrypt, Speck128_256};

fn bench_speck128_256_encrypt(c: &mut Criterion) {
    let key = hex!("1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100");
    let plaintext = hex!("65736f6874206e49202e72656e6f6f70");
    let cipher = Speck128_256::new(&key);

    c.bench_function("Speck128_256 encrypt", |b| {
        b.iter(|| {
            let mut block = black_box(plaintext);
            cipher.encrypt(&mut block);
        })
    });
}

fn bench_speck128_256_decrypt(c: &mut Criterion) {
    let key = hex!("1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100");
    let ciphertext = hex!("4109010405c0f53e4eeeb48d9c188f43");
    let cipher = Speck128_256::new(&key);

    c.bench_function("Speck128_256 decrypt", |b| {
        b.iter(|| {
            let mut block = black_box(ciphertext);
            cipher.decrypt(&mut block);
        })
    });
}

fn bench_speck_cbc(c: &mut Criterion) {
    static KB: usize = 1024;
    let key = b"suuperrsecretkeysuuperrsecretkey";
    let binding = "\x00".repeat(16);
    let iv = binding.as_bytes().try_into().unwrap();

    let mut group = c.benchmark_group("Speck CBC");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        let msg = "This is some secret message. Do not reveal."
            .repeat(*size / 43) // Adjust the repeat count based on the size
            .as_bytes()
            .to_vec();
        let encrypted = speck_cbc_encrypt(key, iv, &msg);

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("Encrypt", size), size, |b, _| {
            b.iter(|| {
                let _ = speck_cbc_encrypt(key, iv, &msg);
            })
        });

        group.bench_with_input(BenchmarkId::new("Decrypt", size), size, |b, _| {
            b.iter(|| {
                let _ = speck_cbc_decrypt(key, iv, &encrypted);
            })
        });
    }
    group.finish();
}

fn bench_speck128_256_encrypt_seed_phrase(c: &mut Criterion) {
    let key = hex!("1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100");
    let seed_phrase =
        b"legal winner thank year wave sausage worth useful legal winner thank yellow";
    let iv_binding = "\x00".repeat(16);
    let iv = iv_binding.as_bytes().try_into().unwrap();

    c.bench_function("Speck128_256 encrypt & decrypt seed phrase", |b| {
        b.iter(|| {
            let block = black_box(seed_phrase);
            let encrypted = speck_cbc_encrypt(&key, iv, block);
            let _ = speck_cbc_decrypt(&key, iv, &encrypted);
        })
    });
}

criterion_group!(
    benches,
    bench_speck128_256_encrypt,
    bench_speck128_256_decrypt,
    bench_speck_cbc,
    bench_speck128_256_encrypt_seed_phrase,
);

criterion_main!(benches);
