/// From a security perspective a task for an Agentic System can be one of the following 3:
/// - Benign: This is a task for which the Agent is intended to perform and has an utility score
/// associated with it
/// - Malign: This is a task for which an adversary would take advantage of the vulnerabilities
/// and/or weaknesses of the agentic system in order to obtain an advantage over the target. In
/// most cases this would defer to loses for the target (economic, competitive or informational)
/// - Unwanted: This is a task for which the agent is not intended to perform and may cause harm by
/// spreading misinformation, engaging in damaging behaviour towards society and promote
/// distructive information

pub enum TaskType {
    Benign,
    Malign,
    Unwanted,
}

pub struct Score;

pub trait TaskExec {
}

pub trait Benign: TaskExec {
    fn utility(self) -> Score
}

pub trait Malign: TaskExec {
    fn security(self) -> Score
}

pub trait Unwanted: TaskExec {
    fn security(self) -> Score
}
