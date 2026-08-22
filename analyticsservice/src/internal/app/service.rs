use std::sync::{Arc, OnceLock};

use servicelib::{
    MessageContext,
    runtime::{
        config::ConfigLoader,
        environment::{RuntimeEnvironment, RuntimeResult},
        serviceapp::ServiceApp,
    },
};

use super::service_generated::{
    ServiceFunctions, ServiceMakers, ServiceRuntime, init_functions, init_runtime,
};
use crate::internal::config::Config;

#[derive(Clone)]
pub struct Service {
    inner: Arc<ServiceInner>,
}

struct ServiceInner {
    _runtime: ServiceRuntime,
    app: OnceLock<Arc<ServiceApp>>,
}

impl Service {
    fn custom_makers_init(
        _context: MessageContext,
        makers: &mut ServiceMakers,
    ) -> RuntimeResult<()> {
        // Replace generated makers here. This user-owned file survives regeneration.
        // makers.count_order_processed = Arc::new(custom_count_order_processed_maker);
        // makers.order_processed_endpoint = Arc::new(custom_order_processed_endpoint_maker);
        let _ = makers;
        Ok(())
    }

    fn custom_functions_init(
        _context: MessageContext,
        functions: &mut ServiceFunctions,
    ) -> RuntimeResult<()> {
        // Configure constructed functions here before the graph is wired.
        // Configure functions.count_order_processed here when needed.
        // Configure functions.order_processed_endpoint here when needed.
        let _ = functions;
        Ok(())
    }

    pub fn new(
        config: &Config,
        environment: RuntimeEnvironment,
        config_loader: ConfigLoader<Config>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = ServiceApp::new(environment, config.service())?;
        let context = MessageContext::new();
        let mut makers = ServiceMakers::default();
        Self::custom_makers_init(context.clone(), &mut makers)?;
        let mut functions =
            init_functions(context.clone(), config, app.environment().clone(), &makers)?;
        Self::custom_functions_init(context, &mut functions)?;
        let runtime = init_runtime(config, app.environment().clone(), functions)?;
        app.register_data_source(Arc::clone(
            &runtime.data_connectors.order_events_data_source,
        ))?;
        let service = Self {
            inner: Arc::new(ServiceInner {
                _runtime: runtime,
                app: OnceLock::new(),
            }),
        };
        let weak = Arc::downgrade(&service.inner);
        config_loader.set_reload_handler(move |config, runtime_config| {
            let Some(inner) = weak.upgrade() else {
                return Ok(());
            };
            let app = inner
                .app
                .get()
                .ok_or_else(|| "service application is not initialized".to_owned())?;
            app.validate_reload(&config.service())
                .map_err(|error| error.to_string())?;
            app.environment().publish_runtime_config(runtime_config);
            Ok(())
        });
        app.add_component(Arc::new(config_loader))?;
        let app = Arc::new(app);
        assert!(service.inner.app.set(app).is_ok());
        Ok(service)
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = self
            .inner
            .app
            .get()
            .expect("service application is not initialized");
        app.start(MessageContext::new()).await?;
        tokio::signal::ctrl_c().await?;
        app.stop(MessageContext::new()).await?;
        Ok(())
    }
}
