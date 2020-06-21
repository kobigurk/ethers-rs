use crate::{JsonRpcClient, Provider, Ws};

use ethers_core::types::U256;
use async_tungstenite::tungstenite::{self, protocol::Message};
use futures_util::{sink::Sink, stream::Stream};
use serde_json::Value;

pub trait PubsubClient: JsonRpcClient {
    type Notifications;
    fn notifications(&mut self) -> &mut Self::Notifications;
    fn push_notification<T>(notification_id: U256, data: T) {

    }
}


//  struct Notification {
//  id: U256, notifcation id
//  rx: Rx // the sink
//  
//  impl Stream for Notification

// Add special methods to the provider for the subsub implementers
// impl<P: PubsubClient> Provider<P> {
//     async fn subscribe(&self, args: Value) -> 
// 
//         Ok(self
//             .0
//             .request("eth_subscribe", [arg])
//             .await
//             .map_err(Into::into)?)
// 
// }
