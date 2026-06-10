use criterion::measurement::Measurement;
use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_types::dis::DisOptions;
use rspirv2_types::module::Module;
use std::fs;
use std::hint::black_box;
use std::io::Write;
use std::time::Duration;

/// `core` crate dumped from rust-gpu before it is linked
///
/// Contains a ton of strings of the various symbol names and debug source code, in addition to custom rust-gpu
/// `ExtInstSet` instructions to represent stack frames and line numbers.
fn dis_core_pre_link(c: &mut Criterion) {
    let bytes = fs::read(spv::CORE_PRE_LINK.spv()).unwrap();
    let mut g = c.benchmark_group("dis core_pre_link");
    g.sample_size(100);
    g.measurement_time(Duration::from_secs(10));
    g.bench_function("dis core_pre_link", |b| dis(b, &bytes));
}

/// A spirv artifact from glslc *without* debug symbols, mostly actual instructions
fn dis_texture_grad_offset(c: &mut Criterion) {
    let bytes = fs::read(spv::TEXTURE_GRAD_OFFSET.spv()).unwrap();
    let mut g = c.benchmark_group("dis texture_grad_offset");
    g.bench_function("dis texture_grad_offset", |b| dis(b, &bytes));
}

/// A spirv artifact from rust-gpu *with* rust debug symbols
///
/// So there will be a few giant strings of source code
fn dis_dis_reference(c: &mut Criterion) {
    let bytes = fs::read(spv::DIS_REFERENCE.spv()).unwrap();
    let mut g = c.benchmark_group("dis dis_reference");
    g.bench_function("dis dis_reference", |b| dis(b, &bytes));
}

fn dis(b: &mut Bencher<'_, impl Measurement>, bytes: &[u8]) {
    let module = Module::<CoreInstSet>::from_bytes_unchecked(black_box(bytes)).unwrap();
    b.iter(|| {
        let dis = module
            .inst
            .as_raw_slice()
            .dis::<CoreInstSet>(DisOptions::simple());
        let mut stdout = Vec::new();
        write!(&mut stdout, "{}", dis).unwrap();
        stdout
    });
}

criterion_group!(
    benches,
    dis_texture_grad_offset,
    dis_core_pre_link,
    dis_dis_reference,
);
criterion_main!(benches);
