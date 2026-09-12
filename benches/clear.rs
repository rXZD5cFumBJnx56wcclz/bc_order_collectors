mod prelude;

use std::hint::black_box;

use prelude::*;

use bc_order_collectors::clear::CLEAR;

static COLLECTOR: LazyLock<CLEAR> = LazyLock::new(|| CLEAR);

fn clear_1(c: &mut Criterion) {
    let trade_cell = TradeState::new(100.);
    c.bench_function("clear_1", |b| {
        b.iter(|| COLLECTOR.collect_orders(black_box(&trade_cell)))
    });
}

criterion_group!(benches, clear_1);
criterion_main!(benches);
