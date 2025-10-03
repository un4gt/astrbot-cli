#![allow(dead_code)]

use crate::{
    iprintln,
    utils::{build_client, strf_timestamp},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub name: String,
    pub count: u128,
    pub timestamp: f64,
}
#[derive(Debug, Deserialize)]
pub struct RunningStat {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}
#[derive(Debug, Deserialize)]
pub struct MemoryStat {
    pub process: u32,
    pub system: u32,
}
#[derive(Debug, Deserialize)]
pub struct Stat {
    pub platform: Vec<Platform>,
    pub message_count: u128,
    pub platform_count: u32,
    pub plugin_count: u32,
    pub plugins: serde_json::Value,
    pub message_time_series: serde_json::Value,
    pub running: RunningStat,
    pub memory: MemoryStat,
    pub cpu_percent: f32,
    pub thread_count: u32,
    pub start_time: u128,
}

impl Stat {
    pub fn pretty_print(&self) {
        let start_time = strf_timestamp(self.start_time as i64)
            .unwrap_or_else(|_| "Invalid timestamp".to_string());
        println!("开始时间: {}", start_time);
        println!("消息平台:");
        for platform in &self.platform {
            let time = strf_timestamp(platform.timestamp as i64)
                .unwrap_or_else(|_| "Invalid timestamp".to_string());
            println!(
                "  {}: {} (最后更新 {})",
                platform.name, platform.count, time
            );
        }
        println!("消息总数: {}", self.message_count);
        println!("插件数量: {}", self.plugin_count);
        println!("已运行:");
        println!(
            "{}小时{}分{}秒",
            self.running.hours, self.running.minutes, self.running.seconds
        );
        println!("内存占用:");
        println!("  进程: {}", self.memory.process);
        println!("  系统: {}", self.memory.system);
        println!("CPU 负载: {}", self.cpu_percent);
    }
}

pub async fn handle_stat() -> anyhow::Result<Stat> {
    iprintln!("Fetching stat...");

    let api_client = build_client()?;
    let stat = api_client.get_stat().await?;
    Ok(stat)
}
