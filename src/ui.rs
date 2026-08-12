use console::{style, Term};
use crate::ui;
use crate::proc::ProcessInfo;
use std::io;

pub fn format_kb(kb: u32) -> String {
    format!("{:.2} MB", kb as f64 / 1024.0)
}

pub fn clear_screen(term: &Term){
    match term.clear_screen() {
        Ok(_) => (),
        Err(_) => println!("Something went wrong!"),
    }
}

pub fn read_usr_char(term: &Term) -> char {
    match term.read_char(){
        Ok(usr_char) => usr_char,
        Err(_) => {
            println!("Something went wrong!");
            ' '
        }
    }
}

pub fn num_for_kill() -> Option<usize> {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Something went wrong");
    num.trim().parse::<usize>().ok()
}

pub fn result_output(processes: &Vec<ProcessInfo>, max_size: usize){
    
    for (i, proc) in processes.iter().take(max_size).enumerate() {
        let rss_str = ui::format_kb(proc.rss_kb);
        let rss_colored = if proc.rss_kb > 1_024_000 {
            style(&rss_str).red()
        } 
        else if proc.rss_kb > 256_00 {
            style(&rss_str).yellow()
        } 
        else {
            style(&rss_str).green()
        };
        println!("[{}] {} \n PID = {}:  {}\n===========================",
            i+1,
            proc.name,
            proc.pid,
            rss_colored
        );
    } 

}
