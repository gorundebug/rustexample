use std::sync::{Arc, Mutex};

use servicelib::{
    CallableSubStream, MessageContext, Payload, SubStreamCollectorFunc,
    operators::MapFunction,
    runtime::{
        common::RuntimeStream,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use crate::internal::types::{AnalyticsEvent, AnalyticsResult};

#[derive(Clone)]
pub struct InvokeAnalyticsSubstream {
    substream: Arc<dyn CallableSubStream<AnalyticsEvent, AnalyticsResult>>,
}

impl InvokeAnalyticsSubstream {
    pub fn new(substream: Arc<dyn CallableSubStream<AnalyticsEvent, AnalyticsResult>>) -> Self {
        Self { substream }
    }
}

impl MapFunction<AnalyticsEvent, AnalyticsResult> for InvokeAnalyticsSubstream {
    async fn map(
        &self,
        context: MessageContext,
        _stream: &dyn RuntimeStream,
        value: &AnalyticsEvent,
        out: &impl servicelib::runtime::collector::Collect<AnalyticsResult>,
    ) {
        let results = Arc::new(Mutex::new(Vec::<(MessageContext, AnalyticsResult)>::new()));
        let collected = Arc::clone(&results);
        self.substream
            .consume(
                context,
                value.clone(),
                Arc::new(SubStreamCollectorFunc(
                    move |result_context: MessageContext, result: Payload<AnalyticsResult>| {
                        let collected = Arc::clone(&collected);
                        async move {
                            collected
                                .lock()
                                .expect("analytics SubStream result lock poisoned")
                                .push((result_context, result.into_value()));
                            true
                        }
                    },
                )),
            )
            .await
            .expect("analytics SubStream invocation failed");
        let results = std::mem::take(
            &mut *results
                .lock()
                .expect("analytics SubStream result lock poisoned"),
        );
        for (result_context, result) in results {
            out.collect(result_context, result).await;
        }
    }
}

pub async fn make_invoke_analytics_substream(
    _context: MessageContext,
    _environment: RuntimeEnvironment,
) -> RuntimeResult<InvokeAnalyticsSubstream> {
    Err(
        servicelib::runtime::environment::RuntimeError::InvalidConfiguration(
            "InvokeAnalyticsSubstream requires the generated SubStream handle".into(),
        ),
    )
}
