use std::thread;
use std::time::Duration;

use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind, get_current_pid};

const SAMPLE_INTERVAL: Duration = Duration::from_secs(3);
const CPU_SAMPLE_DELAY: Duration = Duration::from_millis(250);

fn process_refresh_kind() -> ProcessRefreshKind {
    ProcessRefreshKind::nothing()
        .with_cpu()
        .with_memory()
        .with_exe(UpdateKind::OnlyIfNotSet)
}

fn format_mb(bytes: u64) -> String {
    let megabytes = bytes as f64 / (1024.0 * 1024.0);
    if megabytes >= 1024.0 {
        format!("{:.2} GB", megabytes / 1024.0)
    } else {
        format!("{megabytes:.1} MB")
    }
}

fn is_launcher_helper(process: &Process) -> bool {
    let name = process.name().to_string_lossy().to_ascii_lowercase();
    let exe = process
        .exe()
        .and_then(|path| path.file_name())
        .map(|name| name.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    let candidate = if exe.is_empty() { name } else { exe };

    [
        "msedgewebview2.exe",
        "msedgewebview2",
        "webview2",
        "webviewhost.exe",
        "webviewhost",
    ]
    .iter()
    .any(|needle| candidate.contains(needle))
}

fn collect_launcher_group(system: &System, root_pid: Pid) -> Vec<Pid> {
    let mut result = vec![root_pid];
    let mut stack = vec![root_pid];

    while let Some(parent_pid) = stack.pop() {
        for (pid, process) in system.processes() {
            if process.parent() != Some(parent_pid) {
                continue;
            }

            if *pid == root_pid || is_launcher_helper(process) {
                result.push(*pid);
                stack.push(*pid);
            }
        }
    }

    result
}

fn print_sample(system: &mut System, root_pid: Pid) {
    let tracked_pids = collect_launcher_group(system, root_pid);

    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&tracked_pids),
        true,
        process_refresh_kind(),
    );
    thread::sleep(CPU_SAMPLE_DELAY);
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&tracked_pids),
        true,
        process_refresh_kind(),
    );

    if system.process(root_pid).is_none() {
        println!("[perf] launcher process not available for sampling");
        return;
    }

    let mut cpu_percent = 0.0;
    let mut memory = 0_u64;
    let mut virtual_memory = 0_u64;

    for pid in &tracked_pids {
        if let Some(process) = system.process(*pid) {
            cpu_percent += process.cpu_usage();
            memory += process.memory();
            virtual_memory += process.virtual_memory();
        }
    }

    println!(
        "[perf] launcher group root_pid={} | tracked_processes={} | cpu={:.1}% | ram={} | vmem={}",
        root_pid.as_u32(),
        tracked_pids.len(),
        cpu_percent,
        format_mb(memory),
        format_mb(virtual_memory)
    );
}

pub fn start_dev_monitor() {
    thread::spawn(|| {
        let mut system =
            System::new_with_specifics(RefreshKind::nothing().with_processes(process_refresh_kind()));
        let root_pid = match get_current_pid() {
            Ok(pid) => pid,
            Err(_) => {
                println!("[perf] failed to resolve current process id");
                return;
            }
        };

        println!(
            "[perf] dev resource monitor started; sampling launcher process group every {}s (root pid={})",
            SAMPLE_INTERVAL.as_secs(),
            root_pid.as_u32()
        );

        loop {
            print_sample(&mut system, root_pid);
            thread::sleep(SAMPLE_INTERVAL);
        }
    });
}
