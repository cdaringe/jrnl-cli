use job_scheduler::{JobScheduler, Job};
use std::process::Command;
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();
    let job = Job::new("0 0 * * * *".parse().unwrap(), || {
      let output = Command::new("terminal-notifier")
        .args(&["-message", "journal"])
        .output()
        .expect("failed to execute");
      println!("status: {}", output.status);
      io::stdout().write_all(&output.stdout).unwrap();
      io::stderr().write_all(&output.stderr).unwrap();
    });
    sched.add(job);
    loop {
      sched.tick();
      let duration = sched.time_till_next_job();
      println!("{} minutes until next job", duration.as_secs() / 60);
      let min_seconds = if duration.as_secs() < 60 { duration.as_secs() } else { 60 };
      std::thread::sleep(Duration::from_secs(min_seconds));
    }
}
