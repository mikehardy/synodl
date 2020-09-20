/*

SynoDL - CLI for Synology's DownloadStation
Copyright (C) 2015 - 2018  Stefan Ott

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

#ifndef __SYNODL_SYNO_H
#define __SYNODL_SYNO_H

#include <inttypes.h>

struct session
{
	char sid[128];
};

struct task
{
	char id[16];
	char fn[128];
	char status[32];
	int64_t size;
	int64_t downloaded;
	int64_t uploaded;
	int speed_dn;
	int speed_up;
	int percent_dn;
};

int syno_login(const char *b, struct session *s, const char *u, const char *p);
int syno_list(const char *base, struct session *s, void (*cb)(struct task *));
int syno_download(const char *base, struct session *s, const char *dl_url);
int syno_logout(const char *base, struct session *s);
int syno_pause(const char *base, struct session *s, const char *ids);
int syno_resume(const char *base, struct session *s, const char *ids);
int syno_delete(const char *base, struct session *s, const char *ids);
#endif
