#!/usr/bin/env python

import json

from tornado.ioloop import IOLoop
from tornado.web import RequestHandler, Application

class AuthHandler(RequestHandler):

	def get(self):

		res = {}
		data = {}

		res['success'] = True

		method = self.get_argument('method')
		if method == 'login':
			data['sid'] = 1

		res['data'] = data
		self.write(json.dumps(res))

class TaskHandler(RequestHandler):

	tasks = [{
		"id": 0,
		"title": "debian-8.6.0-amd64-netinst.iso",
		"status": "downloading",
		"size": 258998272,
		"additional": {
			"transfer": {
				"size_downloaded": 158998272,
				"speed_download": 121021
			}
		}
	},{
		"id": 1,
		"title": "Slackware 14.2 x86_64 DVD ISO",
		"status": "paused",
		"size": 2770253906,
		"additional": {
			"transfer": {
				"size_downloaded": 770253906
			}
		}
	},{
		"id": 2,
		"title": "archbang-011215-i686.iso",
		"status": "seeding",
		"size": 456130560,
		"additional": {
			"transfer": {
				"size_downloaded": 456130560,
				"size_uploaded": 406130560,
				"speed_upload": 83923
			}
		}
	},{
		"id": 3,
		"title": "robolinux64-live-mate-v8.1",
		"status": "seeding",
		"size": 1964947537,
		"additional": {
			"transfer": {
				"size_downloaded": 1964947537,
				"speed_upload": 923
			}
		}
	},{
		"id": 4,
		"title": "KNOPPIX 7.2.0 DVD",
		"status": "finished",
		"size": 4112431185,
		"additional": {
			"transfer": {
				"size_downloaded": 4112431185
			}
		}
	},{
		"id": 5,
		"title": "ubuntu-15.04-desktop-amd64.iso",
		"status": "finished",
		"size": 1148903751,
		"additional": {
			"transfer": {
				"size_downloaded": 1148903751
			}
		}
	},{
		"id": 6,
		"title": "Peppermint-7-20160616-amd64.iso",
		"status": "finished",
		"size": 1105954078,
		"additional": {
			"transfer": {
				"size_downloaded": 1105954078
			}
		}
	}]

	def get(self):

		res = {}
		data = {}

		res['success'] = True;

		method = self.get_argument('method')

		print(method)

		if method == 'create':
			task = {}
			task['id'] = len(self.tasks)
			task['title'] = self.get_argument('uri')
			task['status'] = 'downloading'
			task['size'] = 1234

			transfer = {}
			transfer['size_downloaded'] = 500
			transfer['size_uploaded'] = 250
			transfer['speed_download'] = 1000
			transfer['speed_upload'] = 50

			task['additional'] = { 'transfer': transfer }
			self.tasks.append(task)
		elif method == 'delete':
			id = int(self.get_argument('id'))
			del(self.tasks[id])
		elif method == 'list':
			data['tasks'] = self.tasks

		res['data'] = data
		self.write(json.dumps(res))

application = Application([
	(r"/webapi/auth.cgi", AuthHandler),
	(r"//webapi/auth.cgi", AuthHandler),
	(r"/webapi/DownloadStation/task.cgi", TaskHandler),
	(r"//webapi/DownloadStation/task.cgi", TaskHandler),
])

if __name__ == "__main__":

	application.listen(8888)
	IOLoop.instance().start()
