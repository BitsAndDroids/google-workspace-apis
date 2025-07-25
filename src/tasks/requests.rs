use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::{
    auth::types::GoogleClient,
    utils::request::{PaginationRequestTrait, Request},
};

use super::tasklist::types::TaskLists;

pub struct Uninitialized;
pub struct TaskListMode;
pub struct TasksMode;

trait InitializedMode {}

impl InitializedMode for TaskListMode {}
impl InitializedMode for TasksMode {}

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
}

pub struct TaskRequestBuilder<T = Uninitialized> {
    request: Request,
    _mode: std::marker::PhantomData<T>,
}

impl TaskRequestBuilder<Uninitialized> {
    pub fn new(client: &GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            _mode: std::marker::PhantomData,
        }
    }
    /// Get a list of task lists for the authenticated user.
    /// This does not retrieve the actual tasks in the lists,
    pub fn get_task_lists(self) -> TaskRequestBuilder<TaskListMode> {
        let mut builder = TaskRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://tasks.googleapis.com/tasks/v1/users/@me/lists".to_string();
        builder.request.method = reqwest::Method::GET;
        builder
    }

    /// Get a list of tasks from the specified task list.
    pub fn get_tasks(self, task_list_id: &str) -> TaskRequestBuilder<TasksMode> {
        let mut builder = TaskRequestBuilder {
            request: self.request,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://tasks.googleapis.com/tasks/v1/lists/{task_list_id}/tasks");
        builder.request.method = reqwest::Method::GET;
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
    /// Sets the maximum number of results to return.
    fn max_results(mut self, max: i64) -> Self {
        self.request
            .params
            .insert("maxResults".to_string(), max.to_string());
        self
    }

    /// Sets the page token for pagination. Works with `max_results`.
    fn page_token(mut self, token: &str) -> Self {
        self.request
            .params
            .insert("pageToken".to_string(), token.to_string());
        self
    }
}

impl TaskRequestBuilder<TaskListMode> {
    /// Makes a request to retrieve the task lists.
    pub async fn request(self) -> Result<Option<TaskLists>, Error> {
        self.make_request().await
    }
}

impl TaskRequestBuilder<TasksMode> {
    /// Makes a request to retrieve the tasks from the specified task list.
    pub async fn request<T>(self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.make_request().await
    }
}
