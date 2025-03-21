# synodl – Command-line client for Synology's DownloadStation

synodl is a command-line client for the DownloadStation application found on
Synology storage devices. It allows you to comfortably manage your download
tasks from within a terminal window rather than going through the built-in web
interface.

![synodl](https://code.ott.net/synodl/screenshot.png "synodl 0.1.0")

## Gratitude

This is a fork of the [synodl](https://code.ott.net/synodl) from Stefan Ott, he
deserves all the credit for getting it to work.

I don't even know Rust, but I wanted a quick command-line to fix a common state
my Download Station with a self-signed cert gets in: all the seeding torrents go to "error" state.

I want to resume them all.

So I implemented these changes:

- updated Cargo.toml to modern versions
- new ureq version has easy method to accept all certs, so hacked out all rustls stuff
- hacked in a new `-l` command line switch to list all tasks (as a test, really)
- hacked in a new `-r` command line switch to resume the first 300 error tasks

(I didn't want to learn chunk-iteration in Rust today, "all" tasks was too long a URL, so I just chose 300)

The rest of the functionality is exactly as Stefan left it

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

synodl from this fork will never verify any server certificates - all server
certificates will be trusted, meaning it will immediately work with self-signed
certificates like most Synology DiskStations use.

Note that anyone with basic networking skills may implement a man-in-the-middle
attack and intercept your traffic and steal your password, so you should only use
this forked version of synodl on trusted networks.
