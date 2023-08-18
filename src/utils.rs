use chrono::prelude::*;
use notify_rust::Notification;
use std::collections::VecDeque;

pub fn usage_alert(vec: &VecDeque<f32>, last_notified: &mut u32, last_notification_type: &mut String) {
    let minute_since_last_notification: bool = Local::now().timestamp() as u32 - *last_notified > 60;

    if minute_since_last_notification {
        if avg_higher_than(vec, 50) {
            *last_notified = Local::now().timestamp() as u32;
            *last_notification_type = String::from("high");
            notify("High CPU usage alert!");
        } else {
            if last_notification_type == "high" {    
                *last_notified = Local::now().timestamp() as u32;
                *last_notification_type = String::from("low");
                notify("CPU usage back to normal");
            }
        }
    }
}

fn avg_higher_than(vec_deque: &VecDeque<f32>, avg: i32) -> bool {
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
