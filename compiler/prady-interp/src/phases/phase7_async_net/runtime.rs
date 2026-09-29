// Phase 7 — Async Cooperative Runtime & Task Scheduler
// Provides task queues, green-thread event loop execution, and async/await mechanics.

use std::collections::VecDeque;

pub type TaskId = usize;

pub enum TaskStatus {
    Ready,
    Yielded,
    Completed,
}

pub struct AsyncTask {
    pub id: TaskId,
    pub name: String,
    pub step_fn: Box<dyn FnMut() -> TaskStatus>,
}

pub struct AsyncRuntime {
    task_queue: VecDeque<AsyncTask>,
    next_task_id: usize,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
            next_task_id: 1,
        }
    }

    pub fn spawn<F>(&mut self, name: impl Into<String>, step_fn: F) -> TaskId
    where
        F: FnMut() -> TaskStatus + 'static,
    {
        let id = self.next_task_id;
        self.next_task_id += 1;

        self.task_queue.push_back(AsyncTask {
            id,
            name: name.into(),
            step_fn: Box::new(step_fn),
        });

        id
    }

    /// Run the event loop until all tasks finish.
    pub fn run_until_complete(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            match (task.step_fn)() {
                TaskStatus::Ready | TaskStatus::Yielded => {
                    self.task_queue.push_back(task);
                }
                TaskStatus::Completed => {
                    // Task finished
                }
            }
        }
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}
