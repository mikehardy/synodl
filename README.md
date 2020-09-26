# synodl – Command-line client for Synology's DownloadStation

synodl is a command-line client for the DownloadStation application found on Synology storage devices. It allows you to comfortably manage your download tasks from within a terminal window rather than going through the built-in web interface.

![synodl](https://raw.githubusercontent.com/cockroach/synodl/media/screenshot.png "synodl 0.1.0")


## Getting started

Create a configuration file `.synodl` in your home directory with these entries:

```
user = YOURNAME
password = YOURPASSWORD
url = https://YOUR_DEVICE_ADDRESS:5001/
```

## Using synodl

Calling `synodl` without any additional arguments should show an overview of your current download tasks.
Anything that is passed as a parameter will added as a task to DownloadStation.

## Security

At the moment we ignore SSL certificate errors, i.e. anyone with basic networking skills can intercept your
traffic and steal your password.
