use std::sync::Arc;

use bon::bon;
use serde::{Deserialize, Serialize};

use crate::{client::ClientInner, error::Result};

pub struct Chat {
    pub(crate) inner: Arc<ClientInner>,
}

#[bon]
impl Chat {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        return Self { inner };
    }

    #[builder(finish_fn(name = send))]
    pub async fn get_by_guid(
        &self,
        #[builder(field)] include: Vec<ChatInclude>,
        guid: &str,
    ) -> Result<()> {
        let params = include
            .iter()
            .map(|inc| inc.to_string())
            .reduce(|a, b| format!("{},{}", a, b))
            .map(|s| format!("&with={}", s))
            .unwrap_or_else(String::new);
        let response = self.inner.http.get(format!(
            "{}/api/v1/chat/{}?password={}{}",
            self.inner.server_url, guid, self.inner.password, params
        ));
        self.inner.request_data(response).await
    }
}

// Custom builder methods
#[allow(unused)]
impl<'f1, 'f2, S: chat_get_by_guid_builder::State> ChatGetByGuidBuilder<'f1, 'f2, S> {
    fn include(mut self, inc: ChatInclude) -> Self {
        self.include.push(inc);
        self
    }
    fn include_all(mut self, incs: std::vec::IntoIter<ChatInclude>) -> Self {
        self.include.extend(incs.into_iter());
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatInclude {
    #[serde(rename = "participants")]
    Participants,
    #[serde(rename = "lastmessage")]
    LastMessage,
}

impl ChatInclude {
    pub fn to_string(&self) -> String {
        match self {
            ChatInclude::Participants => "participants".to_string(),
            ChatInclude::LastMessage => "lastmessage".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    use crate::client::Client;

    use super::*;

    #[tokio::test]
    async fn test() {
        let server = MockServer::start_async().await;

        server.mock(|when, then| {
            when.method(GET).path("/api/v1/ping");
            then.status(200).json_body(json!({
                "status": 200,
                "message": "Ping received!",
                "data": "pong"
            }));
        });

        let client = Client::builder()
            .server_url(server.url(""))
            .password("password")
            .build();
        let res = client
            .chats()
            .get_by_guid()
            .guid("asd")
            .include(ChatInclude::LastMessage)
            .include(ChatInclude::Participants)
            .send()
            .await;
    }
}
