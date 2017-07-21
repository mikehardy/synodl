# synodl
Command-line client for Synology's DownloadStation

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
