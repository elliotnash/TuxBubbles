use std::sync::Arc;

use bon::bon;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::ClientInner, error::Result, models, utils::build_option_list};

pub struct Chat {
    pub(crate) inner: Arc<ClientInner>,
}

#[bon]
impl Chat {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        return Self { inner };
    }

    #[builder(finish_fn(name = send))]
    pub async fn get(
        &self,
        #[builder(field)] with_last_message: bool,
        #[builder(field)] with_participants: bool,
        #[builder(field)] with_message_attributed_body: bool,
        #[builder(field)] with_message_info_summary: bool,
        #[builder(field)] with_message_payload_data: bool,
        guid: &str,
    ) -> Result<models::Chat> {
        let with_str = build_option_list! {
            with_last_message => "lastmessage",
            with_participants => "participants",
            with_message_attributed_body => "message.attributed-body",
            with_message_info_summary => "message.message-info-summary",
            with_message_payload_data => "message.payload-data",
        }
        .join(",");
        let with = if with_str.is_empty() {
            String::new()
        } else {
            format!("&with={}", with_str)
        };

        let response = self.inner.http.get(format!(
            "{}/api/v1/chat/{}?password={}{}",
            self.inner.server_url, guid, self.inner.password, with
        ));
        self.inner.request_data(response).await
    }

    #[builder(finish_fn(name = send))]
    pub async fn get_icon(&self, guid: &str) -> Result<Vec<u8>> {
        let req = self.inner.http.get(format!(
            "{}/api/v1/chat/{}/icon?password={}",
            self.inner.server_url, guid, self.inner.password
        ));
        self.inner.request_bytes(req).await
    }

    #[builder(finish_fn(name = send))]
    pub async fn get_count(&self) -> Result<models::ChatCount> {
        let req = self.inner.http.get(format!(
            "{}/api/v1/chat/count?password={}",
            self.inner.server_url, self.inner.password
        ));
        self.inner.request_data(req).await
    }

    #[builder(finish_fn(name = send))]
    pub async fn query(
        &self,
        #[builder(field)] with_last_message: bool,
        #[builder(field)] with_participants: bool,
        #[builder(field)] with_sms: bool,
        #[builder(field)] with_archived: bool,
        #[builder(field)] with_message_attributed_body: bool,
        #[builder(field)] with_message_info_summary: bool,
        #[builder(field)] with_message_payload_data: bool,
        limit: Option<u32>,
        offset: Option<u32>,
        sort: Option<String>, // TODO: This should be an enum
    ) -> Result<Vec<models::Chat>> {
        let with = build_option_list! {
            with_last_message => "lastmessage",
            with_participants => "participants",
            with_sms => "sms",
            with_archived => "archived",
            with_message_attributed_body => "message.attributed-body",
            with_message_info_summary => "message.message-info-summary",
            with_message_payload_data => "message.payload-data",
        };
        let req = self
            .inner
            .http
            .post(format!(
                "{}/api/v1/chat/query?password={}",
                self.inner.server_url, self.inner.password
            ))
            .json(&json!({
                "with": with,
                "limit": limit,
                "offset": offset,
                "sort": sort,
            }));
        self.inner.request_data(req).await
    }
}

// Custom builder methods
#[allow(unused)]
impl<'f1, 'f2, S: chat_get_builder::State> ChatGetBuilder<'f1, 'f2, S> {
    fn with_last_message(mut self) -> Self {
        self.with_last_message = true;
        self
    }
    fn with_participants(mut self) -> Self {
        self.with_participants = true;
        self
    }
    fn with_message_attributed_body(mut self) -> Self {
        self.with_message_attributed_body = true;
        self
    }
    fn with_message_info_summary(mut self) -> Self {
        self.with_message_info_summary = true;
        self
    }
    fn with_message_payload_data(mut self) -> Self {
        self.with_message_payload_data = true;
        self
    }
}
#[allow(unused)]
impl<'f1, S: chat_query_builder::State> ChatQueryBuilder<'f1, S> {
    fn with_last_message(mut self) -> Self {
        self.with_last_message = true;
        self
    }
    fn with_participants(mut self) -> Self {
        self.with_participants = true;
        self
    }
    fn with_sms(mut self) -> Self {
        self.with_sms = true;
        self
    }
    fn with_archived(mut self) -> Self {
        self.with_archived = true;
        self
    }
    fn with_message_attributed_body(mut self) -> Self {
        self.with_message_attributed_body = true;
        self
    }
    fn with_message_info_summary(mut self) -> Self {
        self.with_message_info_summary = true;
        self
    }
    fn with_message_payload_data(mut self) -> Self {
        self.with_message_payload_data = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn query() {
        dotenv::dotenv().ok();
        let server_url = std::env::var("BB_SERVER_URL").expect("BB_SERVER_URL must be set");
        let password = std::env::var("BB_PASSWORD").expect("BB_PASSWORD must be set");

        let client = Client::builder()
            .server_url(server_url)
            .password(password)
            .build();
        let res = client
            .chats()
            .query()
            .limit(100)
            .with_last_message()
            .with_participants()
            .send()
            .await;

        assert!(res.is_ok());
    }
}
