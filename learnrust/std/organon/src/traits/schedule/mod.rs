extern crate std;
use std::time::{ Instant, Duration };

pub trait IsSchedulable: HasCreationTime + HasExpectedDuration {}

pub trait HasCreationTime {
    fn creation_time(&self) -> Instant;
}

pub trait HasExpectedDuration {
    fn expected_duration(&self) -> Duration;
}

pub trait IsScheduled: IsSchedulable + HasArrivalTime + HasRunningState {}

pub trait HasArrivalTime {
    fn arrival_time(&self) -> Instant;
}

pub trait HasRunningState {
    fn running_state(&self) -> RunningState;
}

pub enum RunningState {
    /// Is created but not scheduled
    New,
    /// Is queued for execution
    Ready,
    /// Is being executed
    Running,
    /// Is interrupted and replaced, because of priorities, prerequisites or new time-block
    Waiting,
    /// Is over, successfully or not
    Finished,
    /// Is suspended while (time-block change) or as soon as (pending prerequisite) being interrupted
    WaitingSuspended,
    /// Is suspended while (time-block change) or as soon as (higher priorities) being ready
    ReadySuspended,
}

pub trait IsRunnable: IsScheduled + HasSuspensionTime + HasResumeTime + HasCompletionTime {}

pub trait HasSuspensionTime {
    fn suspension_time(&self) -> Instant;
}

pub trait HasResumeTime {
    fn resume_time(&self) -> Instant;
}

pub trait HasCompletionTime {
    fn completion_time(&self) -> Instant;
}

pub trait IsLoggable: IsRunnable + HasTurnaroundTime + HasWaitingTime + HasBurstTime {}

pub trait HasTurnaroundTime {
    fn turnaround_time(&self) -> Duration;
}

pub trait HasWaitingTime {
    fn waiting_time(&self) -> Duration;
}

pub trait HasBurstTime {
    fn burst_time(&self) -> Duration;
}