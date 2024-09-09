use napi_derive::napi;

#[napi]
pub mod inventory {
    use napi::{bindgen_prelude::Error, Status};
    use steamworks::{InventoryError, InventoryResult, InventoryItem, ItemDefId};

    #[napi(object)]
    pub struct JsInventoryItem {
        pub item_id: f64,
        pub definition: i32,
        pub quantity: u16,
        pub flags: u16,
    }

    impl From<InventoryItem> for JsInventoryItem {
        fn from(item: InventoryItem) -> Self {
            JsInventoryItem {
                item_id: item.item_id.0 as f64,
                definition: item.definition.0,
                quantity: item.quantity,
                flags: item.flags,
            }
        }
    }

    #[napi]
    pub fn get_all_items() -> Result<i32, Error> {
        let client = crate::client::get_client();
        let result = client.inventory().get_all_items();
        match result {
            Ok(inventory_result) => Ok(inventory_result.handle()),
            Err(e) => match e {
                InventoryError::OperationFailed => Err(Error::new(Status::GenericFailure, "Operation failed to complete".to_string())),
                InventoryError::GetResultItemsFailed => Err(Error::new(Status::GenericFailure, "Failed to retrieve result items from inventory".to_string())),
                InventoryError::InvalidInput => Err(Error::new(Status::InvalidArg, "Invalid input provided".to_string())),
            },
        }
    }

    #[napi]
    pub fn get_result_items(result_handle: i32) -> Result<Vec<JsInventoryItem>, Error> {
        let client = crate::client::get_client();
        let result = client.inventory().get_result_items(InventoryResult::new(result_handle));
        match result {
            Ok(inventory_items) => {
                let js_items: Vec<JsInventoryItem> = inventory_items
                    .into_iter()
                    .map(JsInventoryItem::from)
                    .collect();
                Ok(js_items)
            }
            Err(e) => match e {
                InventoryError::OperationFailed => Err(Error::new(Status::GenericFailure, "Operation failed to complete".to_string())),
                InventoryError::GetResultItemsFailed => Err(Error::new(Status::GenericFailure, "Failed to retrieve result items from inventory".to_string())),
                InventoryError::InvalidInput => Err(Error::new(Status::InvalidArg, "Invalid input provided".to_string())),
            },
        }
    }

    #[napi]
    pub fn start_purchase(item_defs: Vec<i32>, quantities: Vec<u32>) -> Result<i64, Error> {
        if item_defs.len() != quantities.len() {
            return Err(Error::new(Status::InvalidArg, "Mismatched item definitions and quantities.".to_string()));
        }

        let client = crate::client::get_client();
        let inventory = client.inventory();

        let item_def_ids: Vec<ItemDefId> = item_defs.into_iter().map(ItemDefId).collect();

        match inventory.start_purchase(&item_def_ids, &quantities) {
            Ok(api_call) => Ok(api_call as i64), // Return SteamAPICall_t as i64 for compatibility with JavaScript
            Err(e) => match e {
                InventoryError::OperationFailed => Err(Error::new(Status::GenericFailure, "Failed to start purchase".to_string())),
                InventoryError::GetResultItemsFailed => Err(Error::new(Status::GenericFailure, "Failed to retrieve result items".to_string())),
                InventoryError::InvalidInput => Err(Error::new(Status::InvalidArg, "Invalid input provided".to_string())),
            },
        }
    }
}