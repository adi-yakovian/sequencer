use async_trait::async_trait;
use starknet_class_manager_types::{ClassManagerRequest, ClassManagerResponse};
use starknet_sequencer_infra::component_definitions::ComponentRequestHandler;
use starknet_sequencer_infra::component_server::{LocalComponentServer, RemoteComponentServer};

use crate::ClassManager;

pub type LocalClassManagerServer =
    LocalComponentServer<ClassManager, ClassManagerRequest, ClassManagerResponse>;
pub type RemoteClassManagerServer =
    RemoteComponentServer<ClassManagerRequest, ClassManagerResponse>;

// TODO(Elin): change the request and response the server sees to raw types; remove conversions and
// unwraps.
#[async_trait]
impl ComponentRequestHandler<ClassManagerRequest, ClassManagerResponse> for ClassManager {
    async fn handle_request(&mut self, request: ClassManagerRequest) -> ClassManagerResponse {
        match request {
            ClassManagerRequest::AddClass(class) => {
                ClassManagerResponse::AddClass(self.0.add_class(class.try_into().unwrap()).await)
            }
            ClassManagerRequest::AddDeprecatedClass(class_id, class) => {
                ClassManagerResponse::AddDeprecatedClass(
                    self.0.add_deprecated_class(class_id, class.try_into().unwrap()),
                )
            }
            ClassManagerRequest::GetExecutable(class_id) => {
                let result = self.0.get_executable(class_id).map(|class| class.try_into().unwrap());
                ClassManagerResponse::GetExecutable(result)
            }
            ClassManagerRequest::GetSierra(class_id) => {
                let result = self.0.get_sierra(class_id).map(|class| class.try_into().unwrap());
                ClassManagerResponse::GetSierra(result)
            }
        }
    }
}
