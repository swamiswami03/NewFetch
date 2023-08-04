# NewFetch
A Fast Linux Fetch For Arch Based System. Currently: Arch,Endeavouros and Manjaro 

## Install
This project uses rust to complile and run  

Paste this command into your terminal to install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Please refer to the offical rust website if you are having problems install rust
```
https://www.rust-lang.org/
```
## Configure

This fetch currently only supports 3 linux distros(Arch, EndeavourOs and Manjaro)  
More distros will be added in the future

To configure the fetch for your specific system
+ open up your code editor of choice
+ open the main.rs file located in the src folder
+ go the the main function
  + comment out the distros that isnt yours with ``` // ```
  + open up the terminal in your code editor and type ``` cargo build --release ``` after commenting out the other distros
  + wait for cargo to build your project
+ go to the ``` /target/release``` in the project directory and place the executable in your ```/home/``` folder
+ you can run your fetch with ```./newfetch``` aslong as you are in your ```/home/``` directory

