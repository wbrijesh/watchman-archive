use chrono::prelude::*;
use notify_rust::Notification;
use std::collections::VecDeque;

pub fn high_cpu_alert(vec: &VecDeque<f32>, last_notified: &mut i64) {
    let minute_since_last_notification: bool = Local::now().timestamp() - *last_notified > 60;

    if minute_since_last_notification && avg_higher_than(vec, 50) {
        *last_notified = Local::now().timestamp() as i64;
        notify("High CPU usage alert!");
    }
}

pub fn avg_higher_than(vec_deque: &VecDeque<f32>, avg: i32) -> bool {
    let mut total: f32 = 0.0;
    for value in vec_deque {
        total = total + value
    }

    if total / vec_deque.len() as f32 > avg as f32 {
        return true;
    }
    return false;
}

fn notify(message: &str) {
    let _ = Notification::new()
        .summary("System Statistics")
        .body(message)
        .timeout(0)
        .show();
}
