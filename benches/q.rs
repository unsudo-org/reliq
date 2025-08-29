use ::criterion as c;
use ::std::hint::black_box as bbox;

fn bench(c: &mut c::Criterion) {
    c.bench_function("example-float f32 add()", |b| {
        b.iter(|| {
            let x = bbox(1.0);
            let y = bbox(1.0);
            let _ = x + y;
        })
    });
    c.bench_function("example-float f32 sub()", |b| {
        b.iter(|| {
            let x = bbox(1.0);
            let y = bbox(1.0);
            let _ = x - y;
        })
    });
    c.bench_function("example-float f32 mul()", |b| {
        b.iter(|| {
            let x = bbox(1.0);
            let y = bbox(1.0);
            let _ = x * y;
        })
    });
    c.bench_function("example-float f32 div()", |b| {
        b.iter(|| {
            let x = bbox(1.0);
            let y = bbox(1.0);
            let _ = x / y;
        })
    });
    c.bench_function("add() u8", |b| {
        b.iter(|| {
            let x: ::reliq::q::Q2<u8> = bbox(1.into());
            let y: ::reliq::q::Q2<u8> = bbox(1.into());
            (x + y).unwrap();
        })
    });
    c.bench_function("sub() u8", |b| {
        b.iter(|| {
            let x: ::reliq::q::Q2<u8> = bbox(1.into());
            let y: ::reliq::q::Q2<u8> = bbox(1.into());
            (x - y).unwrap();
        })
    });
    c.bench_function("mul() u8", |b| {
        b.iter(|| {
            let x: ::reliq::q::Q2<u8> = bbox(1.into());
            let y: ::reliq::q::Q2<u8> = bbox(1.into());
            (x * y).unwrap();
        })
    });
    c.bench_function("div() u8", |b| {
        b.iter(|| {
            let x: ::reliq::q::Q2<u8> = bbox(1.into());
            let y: ::reliq::q::Q2<u8> = bbox(1.into());
            (x / y).unwrap();
        })
    });
    c.bench_function("div() u16", |b| {
        b.iter(|| {
            let x: ::reliq::q::Q2<u16> = bbox(1.into());
            let y: ::reliq::q::Q2<u16> = bbox(1.into());
            (x / y).unwrap();
        });
    });
}

fn point(c: &mut c::Criterion) {
    c.bench_function("create `Point<2, 2, u32>`", |b| {
        b.iter(|| {
            let point: ::reliq::point::Point<2, 2, u32> = bbox([
                25_00,
                30_00
            ]).into();
        });
    });
}

c::criterion_group!(benches, bench, point);
c::criterion_main!(benches);