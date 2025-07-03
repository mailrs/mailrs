use super::app::App;

pub mod query;

#[derive(Default)]
pub struct JobServer {
    jobs: Vec<Box<dyn Job>>,
}

impl std::fmt::Debug for JobServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobServer")
            .field("jobs", &format!("... ({} jobs)", self.jobs.len()))
            .finish_non_exhaustive()
    }
}

impl JobServer {
    pub fn add_job<J: Job>(&mut self, job: J) {
        self.jobs.push(Box::new(job));
    }

    pub fn get_next_ready_job(&mut self) -> Option<Box<dyn Job>> {
        // TODO: Don't do the simple implementation here
        //
        if self.jobs.first_mut().map(|j| j.ready()).unwrap_or(false) {
            return self.jobs.pop();
        }

        tracing::trace!("No job ready");
        None
    }

    pub fn progress_states(&mut self) -> Vec<u8> {
        self.jobs.iter_mut().map(|j| j.progress_state()).collect()
    }
}

pub trait Job: Send + Sync + 'static {
    fn progress_state(&mut self) -> u8;
    fn ready(&mut self) -> bool;

    fn finalize(&mut self, app: &mut App);
}
