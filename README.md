# adns-project

First install the rust language on the required machine

This demonstration is done using kali linux, so the installations steps are below

Insall catgo and rustc

`
sudo apt install cargo
sudo apt install rustc`

Now we need to clone the repo of the project 

`git clone https://github.com/madhumithauppu/adns-project`

After cloning the repo we need to go the the directory

`cd adns-project`

Once we are in project folder we will see the file 

Run the command

`sudo cargo run`

This will execute the program, note root permissions are needed as iptables are being modified and need priviledges


![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/b5950f56-dc7f-483c-9588-4f98fcbcf1d4)

Once the compilation is complete we will see the Rocket interface has launched

![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/130a0900-5415-4e87-af43-59f72a8e5225)

Once it is done we can go the ip address in the terminal or localhost:8000, it will present a dashboard to give the inpput of the ip address

![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/2c9e7863-d172-477e-9c17-8cfd7fb0963f)

For this examplet we will use unt.edu ip address to blocking we found the ip address using ping command and see that the connection was successfull

![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/09099908-7188-40c2-afb7-4b43e2e4546c)

Now we will give the ip address from the ping command to block

![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/6649dbb9-6bc1-4c38-bfaf-3c94134fb744)

We see a response that the ip is blocked 

![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/def65144-4a06-4e02-8bba-a59446cfd32c)

Now lets check if we are able to go to unt.edu on the browser



![image](https://github.com/MadhumithaUppu/adns-project/assets/100417568/e1861977-b5e7-4419-97e5-75934f8a602a)

We can see the response that we are unable to connect and the ip is blocked.







