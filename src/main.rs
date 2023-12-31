use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

mod utils;

fn main() {
    let mut sys = System::new();
    let mut cpu_usage_history: VecDeque<f32> = VecDeque::new();
    let mut last_notified: u32 = 0;
    let mut last_notification_type: String = String::from("");

    loop {
        sys.refresh_cpu();

        let mut avg_cpu_usage: f32 = 0.0;
        for cpu_iter in sys.cpus() {
            avg_cpu_usage = avg_cpu_usage + cpu_iter.cpu_usage();
        }
        avg_cpu_usage = avg_cpu_usage / sys.cpus().len() as f32;

        cpu_usage_history.push_back(avg_cpu_usage);
        if cpu_usage_history.len() > 6 {
            cpu_usage_history.pop_front();
        }

        utils::usage_alert(&cpu_usage_history, &mut last_notified, &mut last_notification_type);

        println!("history: {:#?}", cpu_usage_history);

        sleep(Duration::from_secs(1));
    }
}
