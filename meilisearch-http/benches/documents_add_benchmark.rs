// #[cfg(test)]

// use meilisearch_core::Database;
// use meilisearch_core::{ProcessedUpdateResult, UpdateStatus};
// use serde_json::Value;

// use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

// mod common;

// fn write() {

//     let mut server = common::Server::with_uid("tasks");

//     // 1 - Create the index with no primary_key

//     let body = json!({ "uid": "tasks" });
//     let (response, status_code) = server.create_index(body);
//     assert_eq!(status_code, 201);
//     assert_eq!(response["primaryKey"], json!(null));

// }

// pub fn criterion_benchmark(c: &mut Criterion) {

//     let mut server = common::Server::with_uid("tasks");

//     // 1 - Create the index with no primary_key

//     let body = json!({ "uid": "tasks" });
//     let (response, status_code) = server.create_index(body);
//     assert_eq!(status_code, 201);
//     assert_eq!(response["primaryKey"], json!(null));

//     // let dir = tempfile::tempdir().unwrap();
//     // let database = prepare_database(dir.path());

//     // let reader = database.main_read_txn().unwrap();
//     // let index = database.open_index("bench").unwrap();

//     let mut count = 0;
//     // let query = "I love paris ";

//     // let iter = iter::from_fn(|| {
//     //     count += 1;
//     //     query.get(0..count)
//     // });

//     // let mut group = c.benchmark_group("searching in movies (19654 docs)");
//     // group.sample_size(10);

//     // for query in iter {
//     //     let bench_name = BenchmarkId::from_parameter(format!("{:?}", query));
//     //     group.bench_with_input(bench_name, &query, |b, query| b.iter(|| {
//     //         let builder = index.query_builder();
//     //         builder.query(&reader, query, 0..20).unwrap();
//     //     }));
//     // }
//     // group.finish();
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);