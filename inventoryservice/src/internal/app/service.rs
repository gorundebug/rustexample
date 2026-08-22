use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicU64, Ordering},
};

use example_model::types::{OrderItem, OrderItemResult};
use inventory_service_api::{
    inventoryserviceapi::inventory_service_api_server::{
        InventoryServiceApi, InventoryServiceApiServer,
    },
    processorderitem::{ProcessOrderItemRequest, ProcessOrderItemResponse},
};
use servicelib::{
    MessageContext,
    datasource::grpc::{
        NoStreamingEndpointConsumer, TonicDataSource, make_grpc_no_streaming_endpoint_consumer,
    },
    runtime::{
        config::ConfigLoader,
        environment::{RuntimeEnvironment, RuntimeResult},
        serviceapp::ServiceApp,
    },
};
use tonic::{Request, Response, Status};

use super::service_generated::{
    ServiceFunctions, ServiceMakers, ServiceStreams, init_functions, init_runtime,
};
use crate::internal::config::Config;
use crate::internal::functions::ProcessOrderItem;

type ProcessOrderItemEndpoint = NoStreamingEndpointConsumer<
    (),
    ProcessOrderItemRequest,
    ProcessOrderItemResponse,
    OrderItem,
    OrderItemResult,
    OrderItemResult,
    ProcessOrderItem,
>;

#[derive(Clone)]
pub struct Service {
    inner: Arc<ServiceInner>,
}

struct ServiceInner {
    streams: ServiceStreams,
    process_order_item_endpoint: Arc<ProcessOrderItemEndpoint>,
    requests: AtomicU64,
    app: OnceLock<Arc<ServiceApp>>,
}

impl Service {
    fn custom_makers_init(
        _context: MessageContext,
        makers: &mut ServiceMakers,
    ) -> RuntimeResult<()> {
        // Replace generated makers here. This user-owned file survives regeneration.
        // makers.get_inventory_item_data = Arc::new(custom_get_inventory_item_data_maker);
        // makers.process_order_item = Arc::new(custom_process_order_item_maker);
        let _ = makers;
        Ok(())
    }

    fn custom_functions_init(
        _context: MessageContext,
        functions: &mut ServiceFunctions,
    ) -> RuntimeResult<()> {
        // Configure constructed functions here before the graph is wired.
        // Configure functions.get_inventory_item_data here when needed.
        // Configure functions.process_order_item here when needed.
        let _ = functions;
        Ok(())
    }

    pub fn new(
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
        let runtime = init_runtime(config, app.environment().clone(), functions)?;
        let streams = runtime.streams;
        let process_order_item_endpoint = make_grpc_no_streaming_endpoint_consumer(
            streams.process_order_item.as_ref().clone(),
            runtime.handlers.process_order_item,
        )?;
        let service = Self {
            inner: Arc::new(ServiceInner {
                streams,
                process_order_item_endpoint,
                requests: AtomicU64::new(0),
                app: OnceLock::new(),
            }),
        };
        let grpc = TonicDataSource::from_input(&service.inner.streams.process_order_item)?;
        app.register_data_source(grpc)?;
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
        app.add_grpc_service(InventoryServiceApiServer::new(service.clone()))?;
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

#[tonic::async_trait]
impl InventoryServiceApi for Service {
    async fn process_order_item(
        &self,
        request: Request<ProcessOrderItemRequest>,
    ) -> Result<Response<ProcessOrderItemResponse>, Status> {
        self.inner.requests.fetch_add(1, Ordering::Relaxed);
        let context = MessageContext::from_tonic_request(&request);
        let response = self
            .inner
            .process_order_item_endpoint
            .handle(context, request.into_inner())
            .await
            .map_err(|error| Status::internal(error.to_string()))?;
        Ok(Response::new(response))
    }
}
