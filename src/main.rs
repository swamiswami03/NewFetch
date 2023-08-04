git pull origin mainextern crate sysinfo;
use sysinfo::{System, SystemExt};
use colored::*;

fn main() {
    // idea test
    endeavour_os_side()
}

fn endeavour_os_stack() {
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

    let end1 = "      /\\".blue();
    let end2 = "    //  \\\\   ".blue();
    let end3 = "   //    \\ \\".blue();
    let end4 = " / /     _) )".blue();
    let end5 = "/_/___-- __- ".blue();
    let end6 = " /____--     ".blue();


    // string variables
    let os = "OS".red();
    let hostname = "Host Name".red();
    let kernel = "Kernel".red();
    let memory = "Memory".red();
    let cores = "Cores".red();
    let swap = "Swap".red();

    println!("{end1}");
    println!("{end2}");
    println!("{end3}");
    println!("{end4}");
    println!("{end5}");
    println!("{end6}");

    // OS
    if let Some(SystemName) = system_name {
        println!("{}          {}",os,SystemName);
    } else {
        println!("nothing");
    }

    // Host Name
    if let Some(system_host) = system_host {
        println!("{}   {}",hostname,system_host);
    } else {
        println!("nothing");
    }

    // Kernel
    if let Some(system_kernel) = system_kernel {
        println!("{}      {}",kernel,system_kernel);
    } else {
        println!("nothing");
    }

    // Memory
    println!("{}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);

    // Cores
    println!("{}       {}",cores,sys.cpus().len());

    // Swap
    println!("{}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());

}

fn endeavour_os_side() {

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


    // string variables
    let os = "OS".red();
    let hostname = "Host Name".red();
    let kernel = "Kernel".red();
    let memory = "Memory".red();
    let cores = "Cores".red();
    let swap = "Swap".red();

    let end1 = "      /\\".blue();
    let end2 = "    //  \\\\   ".blue();
    let end3 = "   //    \\ \\".blue();
    let end4 = " / /     _) )".blue();
    let end5 = "/_/___-- __- ".blue();
    let end6 = " /____--     ".blue();



    if let Some(SystemName) = system_name {
        println!("{end1}          {}          {}",os,SystemName);
    } else {
        println!("nothing");
    }

    // Host Name
    if let Some(system_host) = system_host {
        println!("{end2}     {}   {}",hostname,system_host);
    } else {
        println!("nothing");
    }

    // Kernel
    if let Some(system_kernel) = system_kernel {
        println!("{end3}      {}      {}",kernel,system_kernel);
    } else {
        println!("nothing");
    }

    // Memory
    println!("{end4}     {}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);

    // Cores
    println!("{end5}     {}       {}",cores,sys.cpus().len());

    // Swap
    println!("{end6}     {}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());



}

