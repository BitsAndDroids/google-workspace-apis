use reqwest::Error;

use crate::{
    tasks::types::TaskLists,
    utils::request::{PaginationRequestTrait, Request},
};

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
    fn get_task_lists(self) -> Self;
    fn request(self) -> impl Future<Output = Result<Option<TaskLists>, Error>>;
}

pub struct TaskRequestBuilder {
    pub request: Request,
}

impl TaskRequestBuilder {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            request: Request::new(client),
        }
    }
}

impl PaginationRequestTrait for TaskRequestBuilder {
    fn max_results(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    fn page_token(mut self, token: &str) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        todo!()
    }
}

impl TaskRequestBuilderTrait for TaskRequestBuilder {
    type TaskRequestBuilder = TaskRequestBuilder;

    fn get_task_lists(mut self) -> Self {
        self.request.url = "https://tasks.googleapis.com/tasks/v1/users/@me/lists".to_string();
        self
    }

    async fn request(self) -> Result<Option<TaskLists>, Error> {
        let res = self
            .request
            .client
            .get(&self.request.url)
            .query(&self.request.params)
            .send()
            .await?;
        let task_list: Option<TaskLists> = if res.status().is_success() {
            Some(res.json().await?)
        } else {
            None
        };
        Ok(task_list)
    }
}
