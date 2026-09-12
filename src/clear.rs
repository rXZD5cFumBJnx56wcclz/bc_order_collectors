use crate::prelude::*;

pub struct CLEAR;

impl OrderCollector for CLEAR {
    fn collect_orders(&self, state: &TradeState) {
        if state.positions.borrow().is_empty() {
            for v in state.orders.borrow_mut().values_mut() {
                for order in v {
                    order.is_active = false;
                }
            }
            for v in state.orders_trigger.borrow_mut().values_mut() {
                for order in v {
                    order.0.is_active = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

    #[test]
    fn collect_orders_res_1() {
        let trade_state = TRADE_STATE();
        trade_state.positions.borrow_mut().clear();
        assert!(trade_state.positions.borrow().is_empty());
        CLEAR.collect_orders(&trade_state);
        assert!(
            trade_state
                .orders
                .borrow()
                .values()
                .all(|v| v.iter().all(|v| !v.is_active))
        );
        assert!(
            trade_state
                .orders_trigger
                .borrow()
                .values()
                .all(|v| v.iter().all(|v| !v.0.is_active))
        );
    }
}
