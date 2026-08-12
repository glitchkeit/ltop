mod proc;
mod ui;
use std::env;
use console::Term;
use crate::proc::collect_processes;
use std::process::Command;

//Modes.
enum Mode{
    MonitoringMode,
    KillMode,
}

fn main() {
    let term = Term::stdout();
    let mut mode = Mode::MonitoringMode;

    let mut max_size: usize = 5; //Number of processes displayed.

    let args: Vec<String> = env::args().collect();

    for arg in &args[1..]{ //Argument processing
        if arg == "-k" {
            mode = Mode::KillMode;
        }
        else if let Ok(n) = arg.parse::<usize>(){
            max_size = n;
        }
    }

    match mode{ 
        Mode::MonitoringMode => loop{

            let mut processes = collect_processes();
            processes.sort_by(|a, b| b.rss_kb.cmp(&a.rss_kb));

            ui::result_output(&processes, max_size);
            println!("----TYPE R TO UPDATE LIST----\n    --(or q to exit)--");
            let usr_char = ui::read_usr_char(&term);
            if usr_char == 'r' || usr_char == 'R'{
                ui::clear_screen(&term);
                continue;
            }
            else if usr_char == 'q' || usr_char == 'Q' {
                break;
            }  
            
        }

        Mode::KillMode => loop{

            let mut processes = collect_processes();
            processes.sort_by(|a, b| b.rss_kb.cmp(&a.rss_kb));

            ui::result_output(&processes, max_size);
            println!("====ENTER PROCESS NUMBER TO KILL====\n  ----TYPE 0 TO REFRESH LIST----");
            let pid_kill = ui::num_for_kill();

            match pid_kill{
                Some(0) => {
                    ui::clear_screen(&term);
                    continue;
                }

                Some(num) if num <= processes.len() => {
                    let pid = processes[num - 1].pid;
                    
                    match Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .spawn() {
                        Ok(_) => println!("Killed PID {}", pid),
                        Err(err) => println!("Failed: {}", err)
                    }
                    println!("Press any key...");
                    ui::read_usr_char(&term);
                }

                _ => {
                    println!("Invalid input!")
                }

            }
        }
    }
}



//Here is the old approach to sorting and inserting into the Vec. 
//Instead of gathering all the processes into a single Vec first, 
//we sort them on the fly and insert each one into its correct position if it meets the criteria. 
//I prefer this method, but the AI ​​said it wasn't optimal. 
//So be it.
/*
fn insert_into_list(list: &mut Vec<(u32, u32)>, pid: u32, rss: u32, max_size: usize) {
    if list.is_empty() {
        list.push((pid, rss));
        return;
    }
    for i in 0..list.len() {
        let (_, exist_rss) = list[i];
        if rss > exist_rss {
            list.insert(i, (pid, rss));
            if list.len() > max_size {
                list.pop();
            }
            return;
        }
    }

    if list.len() < max_size {
        list.push((pid, rss));
    }
}
 */