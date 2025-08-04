use std::collections::HashMap;

use anyhow::{anyhow, Error};
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
pub struct TaskPatchMode;

trait InitializedGetMode {}

impl InitializedGetMode for TaskListMode {}
impl InitializedGetMode for TasksMode {}

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
}

pub struct TasksClient<'a, T = Uninitialized> {
    request: Request<'a>,
    task: Option<Task>,
    _mode: std::marker::PhantomData<T>,
}

impl<'a> TasksClient<'a, Uninitialized> {
    pub fn new(client: &'a mut GoogleClient) -> Self {
        Self {
            request: Request::new(client),
            task: None,
            _mode: std::marker::PhantomData,
        }
    }
    /// Get a list of task lists for the authenticated user.
    /// This does not retrieve the actual tasks in the lists,
    pub fn get_task_lists(self) -> TasksClient<'a, TaskListMode> {
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
    pub fn get_tasks(self, task_list_id: &str) -> TasksClient<'a, TasksMode> {
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

    pub fn insert_task(self, task_list_id: &str) -> TasksClient<'a, TaskInsertMode> {
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

    pub fn complete_task(
        self,
        task_id: &str,
        task_list_id: &str,
    ) -> TasksClient<'a, TaskPatchMode> {
        let mut builder = TasksClient {
            request: self.request,
            task: None,
            _mode: std::marker::PhantomData,
        };
        builder.request.url =
            format!("https://tasks.googleapis.com/tasks/v1/lists/{task_list_id}/tasks/{task_id}");
        builder.request.method = reqwest::Method::PATCH;
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("task".to_string(), task_id.to_string());
        params.insert("taskList".to_string(), task_list_id.to_string());
        builder.request.params = params;
        let payload = serde_json::json!({
            "status": "completed"
        });
        builder.request.body = Some(serde_json::to_string(&payload).unwrap());
        builder
    }
}

impl<'a, T> TasksClient<'a, T> {
    async fn make_request<R>(&mut self) -> Result<Option<R>, Error>
    where
        R: DeserializeOwned,
    {
        self.request.client.refresh_acces_token_check().await?;
        match self.request.method {
            Method::GET => {
                let res = self
                    .request
                    .client
                    .req_client
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
                    .req_client
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

            Method::PATCH => {
                let res = self
                    .request
                    .client
                    .req_client
                    .patch(&self.request.url)
                    .body(self.request.body.clone().unwrap_or_default())
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

impl<'a, T: InitializedGetMode> PaginationRequestTrait for TasksClient<'a, T> {
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

impl<'a> TasksClient<'a, TaskListMode> {
    /// Makes a request to retrieve the task lists.
    pub async fn request(&mut self) -> Result<Option<TaskLists>, Error> {
        self.make_request().await
    }
}

/// A client for interacting with the Google Tasks API in retrieval mode.
///
/// This client allows querying task lists with various filtering options such as
/// completion status, due dates, and visibility settings.
///
/// # Example
/// ```
/// let client = TasksClient::new(client);
/// let tasks = client.show_completed(true).get_due_min(some_date).request().await?;
/// ```
impl<'a> TasksClient<'a, TasksMode> {
    /// Makes a request to retrieve the tasks from the specified task list.
    ///
    /// # Returns
    /// * `Result<Option<Tasks>, Error>` - A result containing the tasks if successful,
    ///   or an error if the request failed. Returns `None` if no tasks were found.
    pub async fn request(&mut self) -> Result<Option<Tasks>, Error> {
        self.make_request().await
    }
    /// Filter tasks by completion date to include only tasks completed before the specified date.
    ///
    /// # Arguments
    /// * `completed_max` - The upper bound (exclusive) for a task's completion date to filter by
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn get_completed_max(mut self, completed_max: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("completedMax".to_string(), completed_max.to_string());
        self
    }

    /// Filter tasks by completion date to include only tasks completed after the specified date.
    ///
    /// # Arguments
    /// * `completed_min` - The lower bound (inclusive) for a task's completion date to filter by
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn get_completed_min(mut self, completed_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("completedMin".to_string(), completed_min.to_string());
        self
    }

    /// Filter tasks by due date to include only tasks due before the specified date.
    ///
    /// # Arguments
    /// * `due_max` - The upper bound (exclusive) for a task's due date to filter by
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn get_due_max(mut self, due_max: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("dueMax".to_string(), due_max.to_string());
        self
    }

    /// Filter tasks by due date to include only tasks due after the specified date.
    ///
    /// # Arguments
    /// * `due_min` - The lower bound (inclusive) for a task's due date to filter by
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn get_due_min(mut self, due_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("dueMin".to_string(), due_min.to_string());
        self
    }

    /// Control whether completed tasks are included in the result.
    ///
    /// # Arguments
    /// * `show_completed` - If true, completed tasks are included in the result
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn show_completed(mut self, show_completed: bool) -> Self {
        self.request
            .params
            .insert("showCompleted".to_string(), show_completed.to_string());
        self
    }

    /// Control whether deleted tasks are included in the result.
    ///
    /// # Arguments
    /// * `show_deleted` - If true, deleted tasks are included in the result
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn show_deleted(mut self, show_due: bool) -> Self {
        self.request
            .params
            .insert("showDeleted".to_string(), show_due.to_string());
        self
    }

    /// Control whether hidden tasks are included in the result.
    ///
    /// # Arguments
    /// * `show_hidden` - If true, hidden tasks are included in the result
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn show_hidden(mut self, show_hidden: bool) -> Self {
        self.request
            .params
            .insert("showHidden".to_string(), show_hidden.to_string());
        self
    }

    /// Filter tasks to include only those updated after the specified time.
    ///
    /// # Arguments
    /// * `updated_min` - The minimum last updated date (inclusive) to filter by
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn get_updated_min(mut self, updated_min: chrono::DateTime<chrono::Utc>) -> Self {
        self.request
            .params
            .insert("updatedMin".to_string(), updated_min.to_string());
        self
    }

    /// Control whether assigned tasks are included in the result.
    ///
    /// # Arguments
    /// * `show_assigned` - If true, assigned tasks are included in the result
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn show_assigned(mut self, show_assigned: bool) -> Self {
        self.request
            .params
            .insert("showAssigned".to_string(), show_assigned.to_string());
        self
    }
}

/// A client for interacting with the Google Tasks API in task insertion mode.
///
/// This client allows creating new tasks with various properties such as
/// title, notes, due dates, and hierarchical placement.
///
/// # Example
/// ```
/// let client = TasksClient::new(client);
/// let task = client.set_task_title("New Task").set_task_notes("Details").request().await?;
/// ```
impl<'a> TasksClient<'a, TaskInsertMode> {
    /// Makes a request to create a task with the specified properties.
    ///
    /// # Returns
    /// * `Result<Option<Tasks>, Error>` - A result containing the created task if successful,
    ///   or an error if the request failed.
    pub async fn request(&mut self) -> Result<Option<Tasks>, Error> {
        self.make_request().await
    }

    /// Sets the parent task for this task, establishing a hierarchical relationship.
    ///
    /// # Arguments
    /// * `parent_id` - The ID of the parent task
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn set_parent(mut self, parent_id: &str) -> Self {
        self.request
            .params
            .insert("parent".to_string(), parent_id.to_string());
        self
    }

    /// Sets the previous sibling task for this task, establishing the order of tasks.
    ///
    /// # Arguments
    /// * `previous_id` - The ID of the previous task in the sequence
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn set_previous(mut self, previous_id: &str) -> Self {
        self.request
            .params
            .insert("previous".to_string(), previous_id.to_string());
        self
    }

    /// Sets the complete task object to be inserted.
    ///
    /// # Arguments
    /// * `task` - The Task object containing all properties to be set
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    pub fn set_task(mut self, task: Task) -> Self {
        self.task = Some(task);
        self
    }

    /// Sets the title of the task to be created.
    ///
    /// # Arguments
    /// * `title` - The title for the task
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_title(mut self, title: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.title = title.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the ETag of the task to be created.
    ///
    /// # Arguments
    /// * `etag` - The ETag for the task
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_etag(mut self, etag: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.etag = etag.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the notes describing the task.
    ///
    /// # Arguments
    /// * `notes` - The notes for the task (max 8192 characters)
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_notes(mut self, notes: &str) -> Self {
        match self.task {
            Some(ref mut task) => task.notes = notes.to_string(),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the due date of the task.
    ///
    /// # Arguments
    /// * `due` - The due date for the task (as a DateTime)
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_due(mut self, due: chrono::DateTime<chrono::Utc>) -> Self {
        match self.task {
            Some(ref mut task) => task.due = Some(due),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the completion date of the task.
    ///
    /// # Arguments
    /// * `completed` - The datetime when the task was completed
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_completed(mut self, completed: chrono::DateTime<chrono::Utc>) -> Self {
        match self.task {
            Some(ref mut task) => task.completed = Some(completed),
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the hidden status of the task.
    ///
    /// # Arguments
    /// * `hidden` - If true, marks the task as hidden
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_hidden(mut self, hidden: bool) -> Self {
        match self.task {
            Some(ref mut task) => task.hidden = hidden,
            None => panic!("Event not initialized for insertion"),
        }
        self
    }

    /// Sets the links associated with the task.
    ///
    /// # Arguments
    /// * `links` - A vector of TaskLink objects to associate with the task
    ///
    /// # Returns
    /// * `Self` - Returns the client for method chaining
    ///
    /// # Panics
    /// * If called before initializing the task
    pub fn set_task_links(mut self, links: Vec<TaskLink>) -> Self {
        match self.task {
            Some(ref mut task) => task.links = links,
            None => panic!("Event not initialized for insertion"),
        }
        self
    }
}

impl<'a> TasksClient<'a, TaskPatchMode> {
    /// Makes a request to update the task with the specified properties.
    ///
    /// # Returns
    /// * `Result<Option<Tasks>, Error>` - A result containing the updated task if successful,
    ///   or an error if the request failed.
    pub async fn request(&mut self) -> Result<Option<Task>, Error> {
        self.make_request().await
    }
}
