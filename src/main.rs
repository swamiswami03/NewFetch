use sysinfo::{System, SystemExt};
use colored::*;

fn main() {
    endeavour();
    manjaro();
    arch();
}

fn endeavour() {

    let sys = System::new_all();


    // sys name
    let system_name = sys.name();
    // host
    let system_host = sys.host_name();
    // kernel
    let system_kernel = sys.kernel_version();
    // memory
    let system_total_memory = sys.total_memory();
    let system_total_memory = system_total_memory/ u64::pow(2,30);
    let system_used_memory = sys.used_memory();
    let system_used_memory = system_used_memory/ u64::pow(2,30);
    let system_total_swap = sys.total_swap();
    let system_total_swap = system_total_swap/u64::pow(2,30);
    let system_used_swap = sys.used_swap();
    let system_used_swap = system_used_swap/u64::pow(2,30);
    // uptime
    let seconds_system_uptime = sys.uptime();
    let seconds_system_uptime = seconds_system_uptime as f32;
    let minutes_system_uptime = seconds_system_uptime/ 60.0;
    let minutes_system_uptime = minutes_system_uptime;
    let hours_system_uptime = seconds_system_uptime / 3600.0;
    let hours_system_uptime = hours_system_uptime;



    // string variables
    let os = "OS".red();
    let hostname = "Host Name".red();
    let kernel = "Kernel".red();
    let memory = "Memory".red();
    let cores = "Cores".red();
    let swap = "Swap".red();
    let uptime = "Uptime".red();

    let end1 = "      /\\".blue();
    let end2 = "    //  \\\\   ".blue();
    let end3 = "   //    \\ \\".blue();
    let end4 = " / /     _) )".blue();
    let end5 = "/_/___-- __- ".blue();
    let end6 = " /____--     ".blue();



    if let Some(SystemName) = system_name {
        println!("{end1}         {}          {}",os,SystemName);
    } else {
        println!("nothing");
    }

    // Host Name
    if let Some(system_host) = system_host {
        println!("{end2}    {}   {}",hostname,system_host);
    } else {
        println!("nothing");
    }

    // Kernel
    if let Some(system_kernel) = system_kernel {
        println!("{end3}     {}      {}",kernel,system_kernel);
    } else {
        println!("nothing");
    }

    // Memory
    println!("{end4}    {}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);

    // Cores
    println!("{end5}    {}       {}",cores,sys.cpus().len());

    // Swap
    //println!("{end6}     {}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());

    println!("{end6}    {}      {:.1} Hours        ",uptime,hours_system_uptime);




}

fn manjaro() {

    let sys = System::new_all();


    // sys name
    let system_name = sys.name();
    // host
    let system_host = sys.host_name();
    // kernel
    let system_kernel = sys.kernel_version();
    // memory
    let system_total_memory = sys.total_memory();
    let system_total_memory = system_total_memory/ u64::pow(2,30);
    let system_used_memory = sys.used_memory();
    let system_used_memory = system_used_memory/ u64::pow(2,30);
    let system_total_swap = sys.total_swap();
    let system_total_swap = system_total_swap/u64::pow(2,30);
    let system_used_swap = sys.used_swap();
    let system_used_swap = system_used_swap/u64::pow(2,30);
    // uptime
    let seconds_system_uptime = sys.uptime();
    let seconds_system_uptime = seconds_system_uptime as f32;
    let minutes_system_uptime = seconds_system_uptime/ 60.0;
    let minutes_system_uptime = minutes_system_uptime;
    let hours_system_uptime = seconds_system_uptime / 3600.0;
    let hours_system_uptime = hours_system_uptime;



    // string variables
    let os = "OS".red();
    let hostname = "Host Name".red();
    let kernel = "Kernel".red();
    let memory = "Memory".red();
    let cores = "Cores".red();
    let swap = "Swap".red();
    let uptime = "Uptime".red();

    let end1 = "||||||||| ||||".green();
    let end2 = "||||||||| ||||".green();
    let end3 = "||||      ||||".green();
    let end4 = "|||| |||| ||||".green();
    let end5 = "|||| |||| ||||".green();
    let end6 = "|||| |||| ||||".green();



    if let Some(SystemName) = system_name {
        println!("{end1}   {}          {}",os,SystemName);
    } else {
        println!("nothing");
    }

    // Host Name
    if let Some(system_host) = system_host {
        println!("{end2}   {}   {}",hostname,system_host);
    } else {
        println!("nothing");
    }

    // Kernel
    if let Some(system_kernel) = system_kernel {
        println!("{end3}   {}      {}",kernel,system_kernel);
    } else {
        println!("nothing");
    }

    // Memory
    println!("{end4}   {}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);

    // Cores
    println!("{end5}   {}       {}",cores,sys.cpus().len());

    // Swap
    //println!("{end6}     {}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());

    println!("{end6}    {}     {:.1} Hours        ",uptime,hours_system_uptime);
}

fn arch() {
    let sys = System::new_all();


    // sys name
    let system_name = sys.name();
    // host
    let system_host = sys.host_name();
    // kernel
    let system_kernel = sys.kernel_version();
    // memory
    let system_total_memory = sys.total_memory();
    let system_total_memory = system_total_memory/ u64::pow(2,30);
    let system_used_memory = sys.used_memory();
    let system_used_memory = system_used_memory/ u64::pow(2,30);
    let system_total_swap = sys.total_swap();
    let system_total_swap = system_total_swap/u64::pow(2,30);
    let system_used_swap = sys.used_swap();
    let system_used_swap = system_used_swap/u64::pow(2,30);
    // uptime
    let seconds_system_uptime = sys.uptime();
    let seconds_system_uptime = seconds_system_uptime as f32;
    let minutes_system_uptime = seconds_system_uptime/ 60.0;
    let minutes_system_uptime = minutes_system_uptime;
    let hours_system_uptime = seconds_system_uptime / 3600.0;
    let hours_system_uptime = hours_system_uptime;



    // string variables
    let os = "OS".red();
    let hostname = "Host Name".red();
    let kernel = "Kernel".red();
    let memory = "Memory".red();
    let cores = "Cores".red();
    let swap = "Swap".red();
    let uptime = "Uptime".red();

    let end1 = "      /\\".cyan();
    let end2 = "     /\\ \\".cyan();
    let end3 = "    /    \\".cyan();
    let end4 = "   /  ,,  \\".cyan();
    let end5 = "  /  |  | -\\".cyan();
    let end6 = " /_''    ''_\\".cyan();



    if let Some(SystemName) = system_name {
        println!("{end1}         {}          {}",os,SystemName);
    } else {
        println!("nothing");
    }

    // Host Name
    if let Some(system_host) = system_host {
        println!("{end2}        {}   {}",hostname,system_host);
    } else {
        println!("nothing");
    }

    // Kernel
    if let Some(system_kernel) = system_kernel {
        println!("{end3}       {}      {}",kernel,system_kernel);
    } else {
        println!("nothing");
    }

    // Memory
    println!("{end4}      {}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);

    // Cores
    println!("{end5}     {}       {}",cores,sys.cpus().len());

    // Swap
    //println!("{end6}     {}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());

    println!("{end6}    {}      {:.1} Hours        ",uptime,hours_system_uptime);
}
