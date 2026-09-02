use std::time::Duration;

use async_trait::async_trait;
use servicelib::{
    MessageContext,
    operators::DelayFunction,
    runtime::{
        common::RuntimeStream,
        config::DelayStreamConfig,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::Order;

#[derive(Default)]
pub struct SoftDeadline;

#[async_trait]
impl DelayFunction<Order> for SoftDeadline {
    async fn duration(
        &self,
        context: MessageContext,
        stream: &dyn RuntimeStream,
        _value: &Order,
    ) -> Duration {
        let margin = stream
            .environment()
            .stream_config(stream.id())
            .and_then(|config| config.delay().map(|delay| delay.duration))
            .unwrap_or_default();
        context.deadline().map_or(margin, |deadline| {
            deadline
                .saturating_duration_since(tokio::time::Instant::now())
                .saturating_sub(margin)
        })
    }
}

pub async fn make_soft_deadline(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
    _config: &DelayStreamConfig,
) -> RuntimeResult<SoftDeadline> {
    Ok(SoftDeadline)
}
