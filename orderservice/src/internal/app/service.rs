use std::sync::{Arc, OnceLock, Weak};

use servicelib::{
    MessageContext,
    datasource::http::AxumDataSource,
    runtime::{
        config::{ConfigLoader, HttpDataConnectorConfig},
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
    runtime: ServiceRuntime,
    app: OnceLock<Arc<ServiceApp>>,
}

impl Service {
    fn custom_makers_init(
        _context: MessageContext,
        makers: &mut ServiceMakers,
    ) -> RuntimeResult<()> {
        // Replace generated makers here. This user-owned file survives regeneration.
        // makers.process_order_items = Arc::new(custom_process_order_items_maker);
        // makers.map_order_item_result_to_order_state = Arc::new(custom_map_order_item_result_to_order_state_maker);
        // makers.soft_deadline = Arc::new(custom_soft_deadline_maker);
        // makers.map_to_order_state = Arc::new(custom_map_to_order_state_maker);
        // makers.map_to_order_processed = Arc::new(custom_map_to_order_processed_maker);
        // makers.process_order = Arc::new(custom_process_order_maker);
        // makers.inventory_sink = Arc::new(custom_inventory_sink_maker);
        // makers.order_processed_endpoint = Arc::new(custom_order_processed_endpoint_maker);
        let _ = makers;
        Ok(())
    }

    fn custom_functions_init(
        _context: MessageContext,
        functions: &mut ServiceFunctions,
    ) -> RuntimeResult<()> {
        // Configure constructed functions here before the graph is wired.
        // Configure functions.process_order_items here when needed.
        // Configure functions.process_order here when needed.
        let _ = functions;
        Ok(())
    }

    pub async fn new(
        config: &Config,
        environment: RuntimeEnvironment,
        config_loader: ConfigLoader<Config>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = ServiceApp::new(environment.clone(), config.service())?;
        let context = MessageContext::new();
        let mut makers = ServiceMakers::default();
        Self::custom_makers_init(context.clone(), &mut makers)?;
        let mut functions =
            init_functions(context.clone(), config, app.environment().clone(), &makers)?;
        Self::custom_functions_init(context, &mut functions)?;
        let runtime = init_runtime(config, app.environment().clone(), functions).await?;
        app.register_data_sink(Arc::clone(&runtime.data_connectors.inventory_data_sink))?;
        app.register_data_sink(Arc::clone(&runtime.data_connectors.order_events_data_sink))?;
        runtime
            .handlers
            .process_order
            .set_timeout_ms(config.request_timeout_ms);
        let http_source = AxumDataSource::new(
            app.environment().clone(),
            &HttpDataConnectorConfig {
                id: config.endpoints.process_order.id_data_connector,
                name: "Order Service API".to_owned(),
                host: config.http_host.clone(),
                port: config.http_port,
                address: String::new(),
                use_dedicated_listener: false,
            },
        );
        http_source.add_endpoint(
            runtime.streams.process_order.as_ref().clone(),
            config.endpoints.process_order.clone(),
            runtime.handlers.process_order.clone(),
        )?;
        app.add_http_router(http_source.router())?;
        app.register_data_source(http_source)?;
        let service = Self {
            inner: Arc::new(ServiceInner {
                runtime,
                app: OnceLock::new(),
            }),
        };
        let weak: Weak<ServiceInner> = Arc::downgrade(&service.inner);
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
            inner
                .runtime
                .data_connectors
                .inventory_data_sink
                .reload_address(config.inventory_address.clone())
                .map_err(|error| error.to_string())?;
            inner
                .runtime
                .handlers
                .process_order
                .set_timeout_ms(config.request_timeout_ms);
            app.environment().publish_runtime_config(runtime_config);
            Ok(())
        });
        app.add_component(Arc::new(config_loader))?;
        let app = Arc::new(app);
        assert!(
            service.inner.app.set(app).is_ok(),
            "service application initialized twice"
        );
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
