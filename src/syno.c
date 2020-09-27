/*

SynoDL - CLI for Synology's DownloadStation
Copyright (C) 2015 - 2020  Stefan Ott

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

#include <stdio.h>
#include <string.h>

#include <curl/curl.h>

#include "config.h"

#ifdef HAVE_JSON_C
#include <json-c/json.h>
#include <json-c/json_tokener.h>
#else
#include <json/json.h>
#include <json/json_tokener.h>
#endif

#include "curl.h"
#include "syno.h"
#include "ui.h"

static int
init_string(struct string *s)
{
	s->size = 1;
	s->ptr = malloc(s->size);

	if (!s->ptr)
	{
		fprintf(stderr, "Malloc failed\n");
		return 1;
	}

	return 0;
}

static void
free_string(struct string *s)
{
	free(s->ptr);
}

static int
json_check_success(json_object *obj)
{
	json_object *success;
	int login_success;

	if (!json_object_object_get_ex(obj, "success", &success))
	{
		fprintf(stderr, "Value 'success' missing from %s\n",
						json_object_get_string(obj));
		return 1;
	}

	if (json_object_get_type(success) != json_type_boolean)
	{
		fprintf(stderr, "Invalid value received for 'success'\n");
		return 1;
	}

	login_success = json_object_get_int(success);
	return login_success == 0;
}

static int
json_load_login(json_object *obj, struct session *s)
{
	json_object *data, *sid;

	if (json_check_success(obj) != 0)
	{
		return 1;
	}

	if (!json_object_object_get_ex(obj, "data", &data))
	{
		fprintf(stderr, "Value 'data' missing from %s\n",
						json_object_get_string(obj));
		return 1;
	}

	json_object_object_get_ex(data, "sid", &sid);
	snprintf(s->sid, sizeof(s->sid), "%s", json_object_get_string(sid));

	return 0;
}

static int
json_load_tasks(json_object *obj, void (*cb)(struct task *))
{
	json_object *data, *tasks, *task, *tmp, *additional, *transfer;
	struct task dt;
	unsigned int i;

	if (json_check_success(obj) != 0)
	{
		return 1;
	}

	if (!json_object_object_get_ex(obj, "data", &data))
	{
		fprintf(stderr, "Value 'data' missing from %s\n",
						json_object_get_string(obj));
		return 1;
	}

	if (!json_object_object_get_ex(data, "tasks", &tasks))
	{
		fprintf(stderr, "No tasks found\n");
		return 1;
	}

	if (json_object_get_type(tasks) != json_type_array)
	{
		fprintf(stderr, "Invalid type returned for tasks\n");
		return 1;
	}

	for (i=0; i < (size_t) json_object_array_length(tasks); i++)
	{
		memset(&dt, 0, sizeof(struct task));

		task = json_object_array_get_idx(tasks, i);

		json_object_object_get_ex(task, "id", &tmp);
		snprintf(dt.id, sizeof(dt.id), "%s",
						json_object_get_string(tmp));

		json_object_object_get_ex(task, "title", &tmp);
		snprintf(dt.fn, sizeof(dt.fn), "%s",
						json_object_get_string(tmp));

		json_object_object_get_ex(task, "status", &tmp);
		snprintf(dt.status, sizeof(dt.status), "%s",
						json_object_get_string(tmp));

		json_object_object_get_ex(task, "size", &tmp);
		dt.size = json_object_get_int64(tmp);

		json_object_object_get_ex(task, "additional", &additional);
		if (json_object_object_get_ex(additional, "transfer", &transfer))
		{
			json_object_object_get_ex(transfer,
						"size_downloaded", &tmp);
			dt.downloaded = json_object_get_int64(tmp);

			json_object_object_get_ex(transfer,
						"speed_download", &tmp);
			dt.speed_dn = json_object_get_int(tmp);

			json_object_object_get_ex(transfer,
							"speed_upload", &tmp);
			dt.speed_up = json_object_get_int(tmp);

			if ((dt.size != 0) && (dt.downloaded != 0))
			{
				dt.percent_dn = ((float)dt.downloaded / dt.size)
									* 100;
			}
		}

		json_object_object_get_ex(transfer, "size_uploaded", &tmp);
		dt.uploaded = json_object_get_int64(tmp);

		cb(&dt);
	}

	return 0;
}

static int
json_load_reply(json_object *obj)
{
	if (json_check_success(obj) != 0)
	{
		return 1;
	}

	return 0;
}

static int
session_load(struct string *st, struct session *session)
{
	json_tokener *tok;
	json_object *obj;
	int res;

	tok = json_tokener_new();
	if (!tok)
	{
		fprintf(stderr, "Failed to initialize JSON tokener\n");
		return 1;
	}

	obj = json_tokener_parse_ex(tok, st->ptr, st->size);
	json_tokener_free(tok);

	if (!obj)
	{
		fprintf(stderr, "Failed to decode JSON data\n");
		json_object_put(obj);
		return 1;
	}

	res = json_load_login(obj, session);
	json_object_put(obj);

	return res;
}

static int
tasks_receive(struct string *st, void (*cb)(struct task *))
{
	int res;
	json_tokener *tok;
	json_object *obj;

	tok = json_tokener_new();
	if (!tok)
	{
		fprintf(stderr, "Failed to initialize JSON tokener\n");
		return 1;
	}

	obj = json_tokener_parse_ex(tok, st->ptr, st->size);
	json_tokener_free(tok);

	if (!obj)
	{
		fprintf(stderr, "Failed to decode JSON data\n");
		json_object_put(obj);
		return 1;
	}

	res = json_load_tasks(obj, cb);
	json_object_put(obj);
	return res;
}

static int
parse_reply(struct string *st)
{
	int res;
	json_tokener *tok;
	json_object *obj;

	tok = json_tokener_new();
	if (!tok)
	{
		fprintf(stderr, "Failed to initialize JSON tokener\n");
		return 1;
	}

	obj = json_tokener_parse_ex(tok, st->ptr, st->size);
	json_tokener_free(tok);

	if (!obj)
	{
		fprintf(stderr, "Failed to decode JSON data\n");
		json_object_put(obj);
		return 1;
	}

	res = json_load_reply(obj);
	json_object_put(obj);
	return res;
}

/*
 * "public" functions
 */

int
syno_login(struct cfg *cfg, struct session *s)
{
	char url[1024];
	struct string st;

	printf("Logging in...\n");

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/auth.cgi?api=SYNO.API.Auth"
		"&version=2&method=login&account=%s&passwd=%s"
		"&session=DownloadStation&format=sid",
		cfg->url, cfg->user, cfg->pw);

	if (curl_do(url, cfg, &st) != 0)
	{
		fprintf(stderr, "Login failed\n");
		free_string(&st);
		return 1;
	}

	if (session_load(&st, s) != 0)
	{
		fprintf(stderr, "Login failed\n");
		free_string(&st);
		return 1;
	}

	free_string(&st);

	if (!strcmp(s->sid, ""))
	{
		fprintf(stderr, "Login failed\n");
		return 1;
	}

	return 0;
}

int
syno_logout(struct cfg *cfg, struct session *s)
{
	char url[1024];
	int res;
	struct string st;

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/auth.cgi?api=SYNO.API.Auth"
		"&version=1&method=logout&session=DownloadStation"
		"&_sid=%s", cfg->url, s->sid);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = parse_reply(&st);
	free_string(&st);
	return res;
}

int
syno_list(struct cfg *cfg, struct session *s, void (*cb)(struct task *))
{
	char url[1024];
	int res;
	struct string st;

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/DownloadStation/task.cgi?"
				"api=SYNO.DownloadStation.Task&version=2"
				"&method=list&additional=transfer&_sid=%s",
				cfg->url, s->sid);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = tasks_receive(&st, cb);
	free_string(&st);
	return res;
}

int
syno_download(struct cfg *cfg, struct session *s, const char *dl_url)
{
	char url[1024], *esc;
	int res;
	struct string st;

	init_string(&st);

	esc = curl_escape(dl_url, strlen(dl_url));
	snprintf(url, sizeof(url), "%s/webapi/DownloadStation/task.cgi?"
			"api=SYNO.DownloadStation.Task&version=2&method=create"
			"&uri=%s&_sid=%s", cfg->url, esc, s->sid);
	curl_free(esc);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = parse_reply(&st);
	free_string(&st);
	return res;
}

int
syno_pause(struct cfg *cfg, struct session *s, const char *ids)
{
	char url[1024];
	int res;
	struct string st;

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/DownloadStation/task.cgi?"
				"api=SYNO.DownloadStation.Task&version=1"
				"&method=pause&id=%s&_sid=%s", cfg->url, ids,
				s->sid);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = parse_reply(&st);
	free_string(&st);
	return res;
}

int
syno_resume(struct cfg *cfg, struct session *s, const char *ids)
{
	char url[1024];
	int res;
	struct string st;

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/DownloadStation/task.cgi?"
				"api=SYNO.DownloadStation.Task&version=1"
				"&method=resume&id=%s&_sid=%s", cfg->url, ids,
				s->sid);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = parse_reply(&st);
	free_string(&st);
	return res;
}

int
syno_delete(struct cfg *cfg, struct session *s, const char *ids)
{
	char url[1024];
	int res;
	struct string st;

	init_string(&st);

	snprintf(url, sizeof(url), "%s/webapi/DownloadStation/task.cgi?"
			"api=SYNO.DownloadStation.Task&version=1"
			"&method=delete&id=%s&_sid=%s&force_complete=false",
			cfg->url, ids, s->sid);

	if (curl_do(url, cfg, &st) != 0)
	{
		free_string(&st);
		return 1;
	}

	res = parse_reply(&st);
	free_string(&st);
	return res;
}
