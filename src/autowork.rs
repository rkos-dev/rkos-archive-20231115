use job_scheduler_ng::{Job, JobScheduler, Schedule};
use std::string::String;

fn set_scheduler(cron_expression: String) {
    let mut sched = JobScheduler::new();
    sched.add(Job::new(cron_expression.parse().unwrap(), || {
        println!("hhello");
    }));
}
