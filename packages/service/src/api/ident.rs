use poem_openapi::{OpenApi, param::Query, payload::Json};
use uuid::Uuid;

use crate::{
    api::ApiResult,
    database::{self},
};

pub mod sync;
pub mod whoami;

pub struct IdentApi;

#[OpenApi]
impl IdentApi {
    /// `whoami` is used to get a valid robot ID based on the provided MAC address.
    /// The validation is not done here.
    #[oai(path = "/ident/whoami", method = "post")]
    async fn whoami(
        &self,
        info: Json<whoami::WhoAmI>,
    ) -> Json<whoami::WhoAmIResponse> {
        let uuid = Uuid::new_v4().to_string();
        let robot_name = format!("robot_{}_{}", info.username, info.mac);
        Json(whoami::WhoAmIResponse {
            robot_uuid: uuid,
            robot_name,
        })
    }

    /// The `sync` endpoint allows a robot to register itself with the server.
    /// The input might be constructed from the `whoami` response,
    /// or by robot's local cache.
    #[oai(path = "/ident/sync", method = "post")]
    async fn sync(
        &self,
        info: Json<sync::Sync>,
    ) -> ApiResult<sync::SyncResponse> {
        let db = database::get_database()?;
        if let Err(e) =
            db.register_robot(&info.mac, &info.name, &info.uuid).await
        {
            log::error!("Failed to register robot: {}", e);
            return Ok(Json(sync::SyncResponse { success: false }));
        }
        Ok(Json(sync::SyncResponse { success: true }))
    }

    /// The `retrieve` endpoint allows fetching robot information by robot ID.
    /// This is used for robots to verify their registration status.
    #[oai(path = "/ident/retrieve", method = "get")]
    async fn retrieve(
        &self,
        Query(username): Query<String>,
        Query(mac_address): Query<String>,
    ) -> ApiResult<Option<sync::RetrieveResponse>> {
        let db = database::get_database()?;
        match db.fuzz_search_by_name(&username, &mac_address).await {
            Ok(Some(robot)) => Ok(Json(Some(sync::RetrieveResponse {
                mac: robot.mac,
                name: robot.name,
                uuid: robot.uuid,
            }))),
            Ok(None) => Ok(Json(None)),
            Err(e) => {
                log::error!("Failed to retrieve robot: {}", e);
                Ok(Json(None))
            }
        }
    }
}
