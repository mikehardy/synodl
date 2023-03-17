# synodl – Command-line client for Synology's DownloadStation

synodl is a command-line client for the DownloadStation application found on
Synology storage devices. It allows you to comfortably manage your download
tasks from within a terminal window rather than going through the built-in web
interface.

![synodl](https://code.ott.net/synodl/screenshot.png "synodl 0.1.0")

## Downloading ##

You can download synodl from [code.ott.net/synodl](https://code.ott.net/synodl/).

## Getting started

Create a configuration file `.synodl` in your home directory with these entries:

```
user = YOURNAME
password = YOURPASSWORD
url = https://YOUR_DEVICE_ADDRESS:5001/
```

## Using synodl

Calling `synodl` without any additional arguments should show an overview of
your current download tasks.  Anything that is passed as a parameter will be
added as a task to DownloadStation.

## Secure password

You can keep your password in a secure location if you specify a
`password_command` instead of a `password` in the config file:

```
password_command = gpg --decrypt ~/.synodl.pw
```

## SSL certificate

With the default configuration, synodl will try to verify your SSL certificate
against the system-wide CA certificates in `/etc/ssl/certs/ca-certificates.crt`.
If you want to provide your own CA certificate, add this to your config file:

```
cacert = /path/to/your/ca.cert
```

In case you want synodl to skip certificate validation, use this:

```
cacert = ignore
```

Note that in this case, anyone with basic networking skills can intercept your
traffic and steal your password.
