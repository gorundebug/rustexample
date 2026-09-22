pub mod inventoryserviceapi {
    tonic::include_proto!("inventoryserviceapi");

    pub mod processorderitem {
        tonic::include_proto!("inventoryserviceapi.processorderitem");
    }
}

pub use inventoryserviceapi::processorderitem;
