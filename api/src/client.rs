use bon::{Builder, bon, builder};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    api,
    error::{Error, Result},
    models::response::Response,
};

// use crate::{
//     api::{ChatApi, MessageApi, AttachmentApi, ContactApi, ServerApi},
//     config::ClientConfig,
//     websocket::{EventHandler, WebSocketHandle},
// };

/// Main BlueBubbles API client
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
}

#[derive(Builder, Debug)]
#[builder(builder_type(vis = "pub", name = ClientBuilder), on(String, into), finish_fn(vis = "", name = build_internal))]
pub(crate) struct ClientInner {
    #[builder(default = HttpClient::new())]
    pub http: HttpClient,
    pub server_url: String,
    pub password: String,
}

impl ClientInner {
    pub(crate) async fn request_data<T: for<'a> Deserialize<'a>>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T> {
        let response = request.send().await.map_err(|e| Error::HTTPError(e))?;

        // Parse the structured response
        let api_response = response.bytes().await.map_err(|e| Error::HTTPError(e))?;
        let mut deserializer = serde_json::Deserializer::from_slice(&api_response);
        let json_response: Response<T> = serde_path_to_error::deserialize(&mut deserializer)
            .map_err(|e| Error::DeserializationError(e))?;

        // Extract data or convert error
        match (json_response.data, json_response.error) {
            (Some(data), _) => Ok(data),
            (None, Some(err)) => Err(Error::from(err)),
            (None, None) => Err(Error::UnexpectedResponse(format!(
                "No data or error: {}",
                json_response.message
            ))),
        }
    }

    pub(crate) async fn request(&self, request: reqwest::RequestBuilder) -> Result<()> {
        let response = request.send().await.map_err(|e| Error::HTTPError(e))?;

        // Parse the structured response
        let api_response = response.bytes().await.map_err(|e| Error::HTTPError(e))?;
        let mut deserializer = serde_json::Deserializer::from_slice(&api_response);
        let json_response: Response = serde_path_to_error::deserialize(&mut deserializer)
            .map_err(|e| Error::DeserializationError(e))?;

        // Extract data or convert error
        if let Some(error) = json_response.error {
            Err(Error::from(error))
        } else {
            Ok(())
        }
    }
}

impl Into<Arc<ClientInner>> for Client {
    fn into(self) -> Arc<ClientInner> {
        self.inner
    }
}

impl<S: client_builder::IsComplete> ClientBuilder<S> {
    pub fn build(self) -> Client {
        Client {
            inner: Arc::new(self.build_internal()),
        }
    }
}

#[bon]
impl Client {
    pub fn builder() -> ClientBuilder {
        ClientInner::builder()
    }

    #[builder(finish_fn(name = send))]
    async fn ping(&self) -> Result<String> {
        let response = self.inner.http.get(format!(
            "{}/api/v1/ping?password={}",
            self.inner.server_url, self.inner.password
        ));
        self.inner.request_data(response).await
    }

    /// Access the chats API namespace
    pub fn chats(&self) -> api::Chat {
        api::Chat::new(Arc::clone(&self.inner))
    }

    // /// Access the messages API namespace
    // pub fn messages(&self) -> MessageApi {
    //     MessageApi::new(Arc::clone(&self.inner))
    // }

    // /// Access the attachments API namespace
    // pub fn attachments(&self) -> AttachmentApi {
    //     AttachmentApi::new(Arc::clone(&self.inner))
    // }

    // /// Access the contacts API namespace
    // pub fn contacts(&self) -> ContactApi {
    //     ContactApi::new(Arc::clone(&self.inner))
    // }

    // /// Access the server API namespace
    // pub fn server(&self) -> ServerApi {
    //     ServerApi::new(Arc::clone(&self.inner))
    // }

    // /// Connect to WebSocket and start receiving events
    // pub async fn connect_websocket<H>(
    //     &self,
    //     handler: H,
    // ) -> Result<WebSocketHandle>
    // where
    //     H: EventHandler + 'static,
    // {
    //     websocket::connect(Arc::clone(&self.inner), handler).await
    // }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn ping() {
        let server = MockServer::start_async().await;

        server.mock(|when, then| {
            when.method(GET).path("/api/v1/ping");
            then.status(200).body_from_file("test_data/ping.json");
        });

        let client = Client::builder()
            .server_url(server.url(""))
            .password("password")
            .build();
        let res = client.ping().send().await;
        assert_eq!(res.expect("Ping failed"), "pong");
    }
}
