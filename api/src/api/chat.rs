use std::sync::Arc;

use bon::bon;
use chrono::{DateTime, Utc};
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
        guid: &str,
    ) -> Result<models::Chat> {
        let with_str = build_option_list! {
            with_last_message => "lastmessage",
            with_participants => "participants",
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
    pub async fn get_messages(
        &self,
        #[builder(field)] with_attachment: bool,
        #[builder(field)] with_handle: bool,
        #[builder(field)] with_sms: bool,
        guid: &str,
        after: Option<DateTime<Utc>>,
        before: Option<DateTime<Utc>>,
        limit: Option<u32>,
        offset: Option<u32>,
        sort: Option<models::SortOrder>,
    ) -> Result<Vec<models::Message>> {
        let with_str = build_option_list! {
            with_attachment => "attachment",
            with_handle => "handle",
            with_sms => "sms",
        }
        .join(",");
        let with = if with_str.is_empty() {
            None
        } else {
            Some(with_str)
        };
        let req = self
            .inner
            .http
            .get(format!(
                "{}/api/v1/chat/{}/message?password={}",
                self.inner.server_url, guid, self.inner.password
            ))
            .query(&[
                ("with", with),
                ("after", after.map(|d| d.timestamp().to_string())),
                ("before", before.map(|d| d.timestamp().to_string())),
                ("limit", limit.map(|l| l.to_string())),
                ("offset", offset.map(|o| o.to_string())),
                ("sort", sort.map(|s| s.name().to_string())),
            ]);
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
}
#[allow(unused)]
impl<'f1, 'f2, S: chat_get_messages_builder::State> ChatGetMessagesBuilder<'f1, 'f2, S> {
    fn with_attachment(mut self) -> Self {
        self.with_attachment = true;
        self
    }
    fn with_handle(mut self) -> Self {
        self.with_handle = true;
        self
    }
    fn with_sms(mut self) -> Self {
        self.with_sms = true;
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
    use crate::client::tests::get_test_client;

    #[tokio::test]
    async fn query() {
        let client = get_test_client();

        let res = client
            .chats()
            .query()
            .limit(100)
            .with_last_message()
            .with_participants()
            .send()
            .await;

        res.expect("Failed to query messages");
    }
    #[tokio::test]
    async fn get_messages() {
        let client = get_test_client();

        let res = client
            .chats()
            .get_messages()
            .guid("SMS;-;+12023896015")
            .send()
            .await;

        res.expect("Failed to get messages");
    }
}
