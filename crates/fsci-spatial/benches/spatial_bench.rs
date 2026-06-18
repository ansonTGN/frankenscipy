use criterion::{Criterion, criterion_group, criterion_main};
use fsci_spatial::{RigidTransform, Rotation};

/// Batch point-cloud transform vs mapping the scalar apply (the "original").
/// Quantifies the loop-invariant hoist: apply_many builds the rotation matrix
/// (and, for the rigid transform, the inverse rotation) ONCE instead of per point.
fn bench_transform_batch(c: &mut Criterion) {
    let n = 8192usize;
    let pts: Vec<[f64; 3]> = (0..n)
        .map(|i| {
            let t = i as f64;
            [t * 0.001, (t * 0.7).sin(), (t * 0.3).cos()]
        })
        .collect();
    let mut group = c.benchmark_group("transform_batch");

    let r = Rotation::from_quat([
        0.022_260_026_714_733_816,
        0.439_679_739_540_909_55,
        0.360_423_405_650_355_9,
        0.822_363_171_905_999_4,
    ]);
    group.bench_function("rotation/apply_many", |b| b.iter(|| r.apply_many(&pts)));
    group.bench_function("rotation/map_apply", |b| {
        b.iter(|| pts.iter().map(|&p| r.apply(p)).collect::<Vec<_>>())
    });

    let tf = RigidTransform::from_components([1.0, 2.0, 3.0], r);
    group.bench_function("rigid/apply_many", |b| b.iter(|| tf.apply_many(&pts, false)));
    group.bench_function("rigid/map_apply", |b| {
        b.iter(|| pts.iter().map(|&p| tf.apply(p, false)).collect::<Vec<_>>())
    });

    group.finish();
}

criterion_group!(benches, bench_transform_batch);
criterion_main!(benches);
