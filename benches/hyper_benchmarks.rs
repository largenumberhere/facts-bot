use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::error::Error;
use std::time::Duration;
use tokio::main;
use tokio::runtime::Runtime;

criterion_group!(hyper_benchmarks, criterion_benchmark);
criterion_main!(hyper_benchmarks);

pub fn criterion_benchmark(c: &mut Criterion) {
    let executor = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    assert!(executor.block_on(hyper_client_create()).is_ok());
    assert!(executor
        .block_on(hyper_client_create_and_send_once())
        .is_ok());
    assert!(executor
        .block_on(hyper_client_create_and_send_twice())
        .is_ok());

    let mut group = c.benchmark_group("hyper_cleint");
    group.measurement_time(Duration::from_secs(30));
    group.confidence_level(0.99f64);
    group.sample_size(200);

    group.bench_function("hyper-http-0", |b| {
        b.to_async(&executor).iter(|| hyper_client_create())
    });

    group.bench_function("hyper-http-1", |b| {
        b.to_async(&executor)
            .iter(|| hyper_client_create_and_send_once())
    });

    group.bench_function("hyper-http-2", |b| {
        b.to_async(&executor)
            .iter(|| hyper_client_create_and_send_twice())
    });
}
//180 nano-secconds
pub async fn hyper_client_create() -> Result<(), Box<dyn Error>> {
    let client = hyper::client::Client::new();
    Ok(())
}

//121ms
pub async fn hyper_client_create_and_send_once() -> Result<(), Box<dyn Error>> {
    let client = hyper::client::Client::new();
    let response = client.get("http://google.com".parse()?).await?;
    Ok(())
}

//227ms
pub async fn hyper_client_create_and_send_twice() -> Result<(), Box<dyn Error>> {
    let client = hyper::client::Client::new();
    let response = client.get("http://google.com".parse()?).await?;
    let response = client.get("http://google.com".parse()?).await?;
    Ok(())
}
