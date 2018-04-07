# nm-gui
This is a GUI for interacting with network manager via DBus.

## Build Requirements
In order to build from source you will need GTK.
install directions [here](http://gtk-rs.org/docs-src/requirements)
You will also need to install rust, and ensure the `cargo` command functions properly.


## Building and Running 
This part is simple,
You have two options, 

### Ubuntu
just run:
~~~
make
~~~

This will install dependencies, build using `cargo`, then run using `cargo`.

### Any Where Else
First install Rust and GTK, as stated above in the build requirements.

The project structure was generated using `cargo`.
when installing rust, this command becomes available.

just use:
~~~
cargo run
~~~
Let `cargo` handle the rest.


##TODO
 - Build inside a docker container.