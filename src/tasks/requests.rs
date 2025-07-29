use anyhow::{Error, anyhow};
use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{
    auth::types::GoogleClient,
    utils::request::{PaginationRequestTrait, Request},
};

use super::{
    tasklist::types::TaskLists,
    types::{Task, TaskLink, Tasks},
};

pub struct Uninitialized;
pub struct TaskListMode;
pub struct TaskInsertMode;
pub struct TasksMode;

trait InitializedGetMode {}

impl InitializedGetMode for TaskListMode {}
impl InitializedGetMode for TasksMode {}

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
}

pub struct TasksClient<T = Uninitialized> {
    request: Request,
    task: Option<Task>,
    _mode: std::marker::PhantomData<T>,
}

impl TasksClient<Uninitialized> {
    pub fn new(client: &GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            task: None,
            _mode: std::marker::PhantomData,
        }
    }
    /// Get a list of task lists for the authenticated user.
    /// This does not retrieve the actual tasks in the lists,
    pub fn get_task_lists(self) -> TasksClient<TaskListMode> {
        let mut builder = TasksClient {
            request: self.request,
            task: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url = "https://tasks.googleapis.com/tasks/v1/users/@me/lists".to_string();
        builder.request.method = reqwest::Method::GET;
        builder
    }

    /// Get a list of tasks from the specified task list.
    pub fn get_tasks(self, task_list_id: &str) -> TasksClient<TasksMode> {
        let mut builder = TasksClient {
            request: self.request,
            task: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://tasks.googleapis.com/tasks/v1/lists/{task_list_id}/tasks");
        builder.request.method = reqwest::Method::GET;
        builder
    }

    pub fn insert_task(self, task_list_id: &str) -> TasksClient<TaskInsertMode> {
        let mut builder = TasksClient {
            request: self.request,
            task: Some(Task::new()),
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://tasks.googleapis.com/tasks/v1/lists/{task_list_id}/tasks");
        builder.request.method = reqwest::Method::POST;
        builder
    }
}

impl<T> TasksClient<T> {
    async fn make_request<R>(&self) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        match self.request.method {
            Method::GET => {
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

            Method::POST => {
                let res = self
                    .request
                    .client
                    .post(&self.request.url)
                    .body(serde_json::to_string(&self.task).unwrap())
                    .query(&self.request.params)
                    .send()
                    .await?;

                if res.status().is_success() {
                    Ok(Some(res.json().await?))
                } else {
                    Ok(None)
                }
            }
            _ => Err(anyhow!("Unsupported HTTP method")),
        }
    }
}

impl<T: InitializedGetMode> PaginationRequestTrait for TasksClient<T> {
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

impl TasksClient<TaskListMode> {
    /// Makes a request to retrieve the task lists.
    pub async fn request(self) -> Result<Option<TaskLists>, Error> {
        self.make_request().await
    }
}

impl TasksClient<TasksMode> {
    /// Makes a request to retrieve the tasks from the specified task list.
    pub async fn request(self) -> Result<Option<Tasks>, Error> {
        self.make_request().await
    }

    pub fn get_completed_max(mut self, completed_max: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("completedMax".to_string(), completed_max.to_string());
        self
    }

    pub fn get_completed_min(mut self, completed_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("completedMin".to_string(), completed_min.to_string());
        self
    }

    pub fn get_due_max(mut self, due_max: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("dueMax".to_string(), due_max.to_string());
        self
    }

    pub fn get_due_min(mut self, due_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("dueMin".to_string(), due_min.to_string());
        self
    }

    pub fn show_completed(mut self, show_completed: bool) -> Self {
        self.request
            .params
            .insert("showCompleted".to_string(), show_completed.to_string());
        self
    }

    pub fn show_deleted(mut self, show_due: bool) -> Self {
        self.request
            .params
            .insert("showDeleted".to_string(), show_due.to_string());
        self
    }

    pub fn show_hidden(mut self, show_hidden: bool) -> Self {
        self.request
            .params
            .insert("showHidden".to_string(), show_hidden.to_string());
        self
    }

    pub fn get_updated_min(mut self, updated_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("updatedMin".to_string(), updated_min.to_string());
        self
    }

    pub fn show_assigned(mut self, show_assigned: bool) -> Self {
        self.request
            .params
            .insert("showAssigned".to_string(), show_assigned.to_string());
        self
    }
}

impl TasksClient<TaskInsertMode> {
    /// Makes a request to retrieve the tasks from the specified task list.
    pub async fn request(self) -> Result<Option<Tasks>, Error> {
        self.make_request().await
    }

    pub fn set_parent(mut self, parent_id: &str) -> Self {
        self.request
            .params
            .insert("parent".to_string(), parent_id.to_string());
        self
    }

    pub fn set_previous(mut self, previous_id: &str) -> Self {
        self.request
            .params
            .insert("previous".to_string(), previous_id.to_string());
        self
    }

    pub fn set_task(mut self, task: Task) -> Self {
        self.task = Some(task);
        self
    }

    pub fn set_task_title(mut self, title: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.title = title.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_etag(mut self, etag: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.etag = etag.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_notes(mut self, notes: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.notes = notes.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_due(mut self, due: chrono::DateTime<chrono::Utc>) -> Self {
        match self.task {
            Some(ref mut task) => task.due = Some(due),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_completed(mut self, completed: chrono::DateTime<chrono::Utc>) -> Self {
        match self.task {
            Some(ref mut task) => task.completed = Some(completed),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_hidden(mut self, hidden: bool) -> Self {
        match self.task {
            Some(ref mut task) => task.hidden = hidden,
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    pub fn set_task_links(mut self, links: Vec<TaskLink>) -> Self {
        match self.task {
            Some(ref mut task) => task.links = links,
            None => panic!("Event not initialized for insertion"),
        }
        self
    }
}
