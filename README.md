# NewFetch
A Fast Linux Fetch For Arch Based System. Currently: Arch,Endeavouros and Manjaro 

## Install
This project uses rust to complile and run  

Paste this command into your terminal to install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Please refer to the offical rust website if you are having problems installing rust
```
https://www.rust-lang.org/
```
## Run Newfetch On Terminal Startup
+ go to the ```/target/release``` find the newfetch executable, and make the file as executable
+ place your newfetch executable in your ```/home/``` directory
+ open up your terminal config file
  + ```.bashrc```
  + ```.zshrc```
  + ``` /.config/fish/config.fish ```
+ right click the NewFetch executable and mark the program as executable 
+ add the command ```./newfetch ``` to your config file
