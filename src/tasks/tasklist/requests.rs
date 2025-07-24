use crate::{tasks::types::TaskList, utils::request::Request};

pub trait TaskRequestBuilderTrait {
    type TaskRequestBuilder;
    fn get_task_lists(self) -> Self;
    fn request(self) -> TaskList;
}

pub struct TaskRequestBuilder {
    pub request: Request,
}

impl TaskRequestBuilder {
    pub fn new(url: String, client: reqwest::Client) -> Self {
        Self {
            request: Request::new(url, client),
        }
    }
}

impl TaskRequestBuilderTrait for TaskRequestBuilder {
    type TaskRequestBuilder = TaskRequestBuilder;

    fn get_task_lists(mut self) -> Self {
        self.request.url = "https://tasks.googleapis.com/tasks/v1/users/@me/lists".to_string();
        self
    }

    fn request(self) -> TaskList {}
}
