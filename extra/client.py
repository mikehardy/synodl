#!/usr/bin/env python

import configparser
import os
import requests

class SynoClient:

    def __init__(self, url):

        self._url = url

    def login(self, user, pw):

        url = ("%s/webapi/auth.cgi?api=SYNO.API.Auth" +
            "&version=2&method=login&account=%s&passwd=%s" +
            "&session=DownloadStation&format=sid") % (self._url, user, pw)
        r = requests.get(url, verify=False)

        data = r.json()

        if not data["success"]:
            raise Exception("Login failed: %s" % data)

        self._sid = data["data"]["sid"]
        print("Successfully logged in")

    def list(self):

        url = ("%s/webapi/DownloadStation/task.cgi?" +
            "api=SYNO.DownloadStation.Task&version=2" +
            "&method=list&additional=transfer&_sid=%s") % (self._url, self._sid)
        r = requests.get(url, verify=False)

        data = r.json()
        tasks = data["data"]["tasks"]

        for task in tasks:
            print("-", task["title"])

if __name__ == "__main__":
    
    with open(os.path.join(os.environ["HOME"], ".synodl")) as f:
        data = [x.strip().split("=") for x in f.readlines()]
        config = {x.strip(): y.strip() for [x,y] in data}

    client = SynoClient(config["url"])
    client.login(config["user"], config["password"])
    client.list()
