A simple project that fires up a api listening on all interfaces on port 8000 that executes a shell command when invoked. In this case this is a docker run command.
I don't really know what I'm doing with rust, but I got it to do what I wanted :).

After you clone the repo you can do a cargo run and then hit the ip address of the host it's running on and see the current date provided by the docker run command. So http://127.0.0.1:8000 if the host is your local workstation.
