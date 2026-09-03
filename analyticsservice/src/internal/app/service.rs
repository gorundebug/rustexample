use servicelib::{
    MessageContext,
    runtime::{
        config::ConfigLoader,
        environment::{RuntimeEnvironment, RuntimeResult},
    },
};

use super::service_generated::{
    GeneratedService, ServiceFunctions, ServiceMakers,
};
use crate::internal::config::Config;

/// User-owned service extension surface. Generated graph and transport wiring
/// live in `service.generated.rs`, so graph changes never add concrete entity
/// names to this file.
pub struct Service {
    generated: GeneratedService,
}

impl Service {
    fn custom_makers_init(
        _context: MessageContext,
        makers: &mut ServiceMakers,
    ) -> RuntimeResult<()> {
        // Replace generated makers here. This file survives regeneration.
        let _ = makers;
        Ok(())
    }

    fn custom_functions_init(
        _context: MessageContext,
        functions: &mut ServiceFunctions,
    ) -> RuntimeResult<()> {
        // Configure constructed functions before the graph is wired.
        let _ = functions;
        Ok(())
    }

    pub async fn new(
        config: &Config,
        environment: RuntimeEnvironment,
        config_loader: ConfigLoader<Config>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            generated: GeneratedService::new(
                config,
                environment,
                config_loader,
                Self::custom_makers_init,
                Self::custom_functions_init,
            )
            .await?,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.generated.run().await
    }
}