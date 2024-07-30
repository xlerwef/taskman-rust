use sysinfo::{Pid, ProcessStatus, System};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HANDLE;

use anyhow::Result;
use indexmap::IndexMap;

#[cfg(target_os = "windows")]
type RawHandle = HANDLE;

pub struct MyHandleWrapper(RawHandle);

#[derive(Clone, PartialEq)]
pub struct ProcessInfo {
    pub name: String,
    pub cmd: Vec<String>,
    pub pid: sysinfo::Pid,
    pub user_id: Option<sysinfo::Uid>,
    pub environ: Vec<String>,
    pub(crate) memory: u64,
    pub(crate) virtual_memory: u64,
    pub(crate) parent: Option<Pid>,
    pub status: ProcessStatus,
    pub start_time: u64,
    pub(crate) run_time: u64,
    pub cpu_usage: f32,
    pub(crate) updated: bool,
    pub old_read_bytes: u64,
    pub old_written_bytes: u64,
    pub read_bytes: u64,
    pub written_bytes: u64,
}

impl ProcessInfo {
    pub fn expand(&self) -> Result<IndexMap<String, String>> {
        let mut user_id = "".to_string();
        let mut parent_pid = "".to_string();
        if (self.user_id.as_ref().is_some()) {
            user_id = self.user_id.as_ref().unwrap().to_string();
        }
        if (self.parent.as_ref().is_some()) {
            parent_pid = self.parent.as_ref().unwrap().to_string();
        }
        let map = IndexMap::from([
            ("Name".to_string(), self.name.to_string()),
            ("PID".to_string(), self.pid.to_string()),
            ("User ID".to_string(), user_id),
            ("CMD".to_string(), self.cmd.join(" ")),
            ("Environment".to_string(), self.environ.join(" ")),
            ("Memory".to_string(), (self.memory/1024).to_string() + " KB"),
            ("Virtual Memory".to_string(), (self.virtual_memory/1024).to_string() + " KB"),
            ("Parent".to_string(), parent_pid),
            ("Status".to_string(), self.status.to_string()),
            ("Start Time".to_string(), self.start_time.to_string()),
            ("Run Time".to_string(), self.run_time.to_string()),
            ("CPU Usage".to_string(), (self.cpu_usage
            ).to_string()),
        ]);
        return Ok(map);
    }
}

#[derive(Clone)]
pub struct CPUsageCalculationValues {
    pub(crate) old_process_sys_cpu: u64,
    pub(crate) old_process_user_cpu: u64,
    pub(crate) old_system_sys_cpu: u64,
    pub(crate) old_system_user_cpu: u64,
}
