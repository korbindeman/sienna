use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sienna::{ProcessingImage, builder::PipelineBuilder};
use std::path::Path;

fn process_pi_hq(c: &mut Criterion) {
    let pipeline = PipelineBuilder::new()
        .exposure(0.5)
        .richness(0.3)
        .film_colors()
        .contrast(1.2, 0.6)
        .build();

    c.bench_function("process_pi_hq_photo", |b| {
        b.iter(|| {
            let path = Path::new("samples/pi.png");
            let mut image = ProcessingImage::from_png(path).expect("failed to load image");
            pipeline.process(&mut image);
            black_box(image);
        })
    });
}

criterion_group!(benches, process_pi_hq);
criterion_main!(benches);
