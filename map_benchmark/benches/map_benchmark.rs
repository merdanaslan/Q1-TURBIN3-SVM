use criterion::{criterion_group, criterion_main, Criterion};
use dashmap::DashMap;
use std::collections::HashMap;

fn bench_maps(c: &mut Criterion) {
    // Insert benchmark
    c.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
        })
    });

    c.bench_function("dashmap_insert", |b| {
        b.iter(|| {
            let map = DashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
        })
    });

    // Read benchmark
    c.bench_function("hashmap_read", |b| {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, i);
        }
        
        b.iter(|| {
            for i in 0..1000 {
                map.get(&i);
            }
        })
    });

    c.bench_function("dashmap_read", |b| {
        let map = DashMap::new();
        for i in 0..1000 {
            map.insert(i, i);
        }
        
        b.iter(|| {
            for i in 0..1000 {
                map.get(&i);
            }
        })
    });
}

criterion_group!(benches, bench_maps);
criterion_main!(benches);