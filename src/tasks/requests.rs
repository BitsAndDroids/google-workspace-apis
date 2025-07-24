use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::{
    tasks::types::TaskLists,
    utils::request::{PaginationRequestTrait, Request},
};

pub struct Uninitialized;
pub struct TaskListMode;
pub struct TasksMode;

pub trait InitializedMode {}

impl InitializedMode for TaskListMode {}
impl InitializedMode for TasksMode {}

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
    fn request(self) -> impl Future<Output = Result<Option<TaskLists>, Error>>;
}

pub struct TaskRequestBuilder<T = Uninitialized> {
    pub request: Request,
    _mode: std::marker::PhantomData<T>,
}

impl TaskRequestBuilder<Uninitialized> {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            request: Request::new(client),
            _mode: std::marker::PhantomData,
        }
    }
    pub fn get_task_lists(self) -> TaskRequestBuilder<TaskListMode> {
        let mut builder = TaskRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://tasks.googleapis.com/tasks/v1/users/@me/lists".to_string();
        builder
    }

    pub fn get_tasks(self, task_list_id: &str) -> TaskRequestBuilder<TasksMode> {
        let mut builder = TaskRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://tasks.googleapis.com/tasks/v1/lists/{task_list_id}/tasks");
        builder
    }
}

impl<T> TaskRequestBuilder<T> {
    async fn make_request<R>(&self) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        let res = self
            .request
            .client
            .get(&self.request.url)
            .query(&self.request.params)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(Some(res.json().await?))
        } else {
            Ok(None)
        }
    }
}

impl<T: InitializedMode> PaginationRequestTrait for TaskRequestBuilder<T> {
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

impl TaskRequestBuilder<TaskListMode> {
    pub async fn request(self) -> Result<Option<TaskLists>, Error> {
        self.make_request().await
    }
}

impl TaskRequestBuilder<TasksMode> {
    pub async fn request<T>(self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.make_request().await
    }
}

impl TaskRequestBuilderTrait for TaskRequestBuilder<Uninitialized> {
    type TaskRequestBuilder = TaskRequestBuilder<TaskListMode>;

    async fn request(self) -> Result<Option<TaskLists>, Error> {
        self.make_request().await
    }
}
