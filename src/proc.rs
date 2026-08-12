use std::fs;

pub struct ProcessInfo{
    pub pid: u32,
    pub name: String,
    pub rss_kb: u32,
}

pub fn collect_processes() -> Vec<ProcessInfo>{
    //A Vec that holds all processes.
    let mut processes = Vec::new();
    
    for entry in fs::read_dir("/proc").unwrap(){
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            
            //Get info about process by his pid.
            if let Ok(pid) = name_str.parse::<u32>() { //Is the file name a u32.
                if is_kernel_thread(pid) == false {
                    if let Some(rss) = get_rss(pid) { //Get rss by pid.
                        processes.push(ProcessInfo{ //Add info about our process to Vec.
                            pid,
                            name: get_name(pid),
                            rss_kb: rss
                        });
                    }
                }
            }
        }
    }
    processes
}

fn get_name(pid: u32) -> String {
    let path = format!("/proc/{}/comm", pid);
    let content = fs::read_to_string(&path);
    match content {
        Ok(s) => return s.trim_end().to_string(),
        Err(_) => return String::from("unknown"),
    }
}

fn get_rss(pid: u32) -> Option<u32> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).ok()?;

    for line in content.lines() {
        if line.starts_with("VmRSS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let num = parts[1].parse().ok()?;
            return Some(num);
        }
    }
    None
}

fn is_kernel_thread (pid: u32) -> bool {
    let path = format!("/proc/{}/status", pid);
    let content = match fs::read_to_string(&path){
        Ok(s) => s,
        Err(_) => return false
    };

    for line in content.lines(){
        if line.starts_with("PPid") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Ok(num) = parts[1].parse::<u32>(){
                return num == 2;
            }
        }
    }
    false
}