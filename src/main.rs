use sysinfo::{System, SystemExt};
use colored::*;
use std::env;


fn main() {
    newfetch();
}


fn newfetch() {


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
    //packages
    let installed_packages = std::process::Command::new("pacman")
        .args(&["-Qq"])
        .output()
        .expect("Failed to execute command");
    let package_output = String::from_utf8_lossy(&installed_packages.stdout);
    let package_count = package_output.lines().count();


    // string variables
    let os = "OS".bright_purple();
    let hostname = "Host Name".bright_purple();
    let kernel = "Kernel".bright_purple();
    let packages = "Packages".bright_purple();
    let cores = "Cores".bright_purple();
    let memory = "Memory".bright_purple();
    let uptime = "Uptime".bright_purple();

    // ascii art declaration

    let mut end1: ColoredString = String::new().bright_blue();
    let mut end2: ColoredString = String::new().bright_blue();
    let mut end3: ColoredString = String::new().bright_blue();
    let mut end4: ColoredString = String::new().bright_blue();
    let mut end5: ColoredString = String::new().bright_blue();
    let mut end6: ColoredString = String::new().bright_blue();
    let mut end7: ColoredString = String::new().bright_blue();

    if let Some(name) = system_name {
        if name == "EndeavourOS" {
            end1 = "      /\\".bright_blue();
            end2 = "    //  \\\\   ".bright_blue();
            end3 = "   //    \\ \\".bright_blue();
            end4 = " / /     _) )".bright_blue();
            end5 = "/_/___-- __- ".bright_blue();
            end6 = " /____--     ".bright_blue();
            end7 = "             ".bright_blue();
            println!("{end1}         {}          {}", os, name);

        } else if name == "Arch" {
            end1 = "      /\\".cyan();
            end2 = "     /\\ \\".cyan();
            end3 = "    /    \\".cyan();
            end4 = "   /  ,,  \\".cyan();
            end5 = "  /  |  | -\\".cyan();
            end6 = " /_''    ''_\\".cyan();
            end7 = "             ".cyan();

            println!("{end1}         {}          {}", os, name);

        } else if name == "Manjaro"{
            end1 = "||||||||| ||||".green();
            end2 = "||||||||| ||||".green();
            end3 = "||||      ||||".green();
            end4 = "|||| |||| ||||".green();
            end5 = "|||| |||| ||||".green();
            end6 = "|||| |||| ||||".green();
            end7 = "             ".green();

            println!("{end1}         {}          {}", os, name);

        }  else{
            end1 = "    .--.".cyan();
            end2 = "   |o_o |".cyan();
            end3 = "   |:_/ |".cyan();
            end4 = "  //    \\".cyan();
            end5 = " (|     | )".cyan();
            end6 = "/'/_   _/`\\".cyan();
            end7 = "\\__)=(___/".cyan();

            println!("{end1}         {}          {}", os, name);

        }
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
    // Packages
    println!("{end4}    {}    {}", packages, package_count);

    // Cores
    println!("{end5}    {}       {}",cores,sys.cpus().len());

    // Memory
    println!("{end6}    {}      {:?}/{:?} GB Used ",memory,system_used_memory,system_total_memory);


    // Swap
    //println!("{end6}     {}        {:?}/{:?} Swap Used",swap,sys.used_swap(),sys.total_swap());

    println!("{end7}    {}      {:.1} Hours        ",uptime,hours_system_uptime);
    println!("");


}