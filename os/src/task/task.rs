//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The first_start_running time
    pub task_meta: TaskMeta,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The metadata of a task
#[derive(Copy, Clone)]
pub struct TaskMeta {
    /// The timestamp of the first runnig
    pub time: usize,
    /// syscall called times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskMeta {
    pub fn new() -> TaskMeta {
        TaskMeta {
            time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }
}