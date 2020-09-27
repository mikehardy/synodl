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

#include <pwd.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "cfg.h"
#include "ini.h"

#define UNUSED __attribute__((unused))
#define CACERT "/etc/ssl/certs/ca-certificates.crt"

static int
config_cb(void *user, const char *s UNUSED, const char *name, const char *value)
{
	struct cfg *cf;

	cf = (struct cfg *) user;

	snprintf(cf->cacert, sizeof(cf->cacert), "%s", CACERT);
	cf->verify_cert = 1;

	if (!strcmp(name, "user"))
		snprintf(cf->user, sizeof(cf->user), "%s", value);
	else if (!strcmp(name, "password"))
		snprintf(cf->pw, sizeof(cf->pw), "%s", value);
	else if (!strcmp(name, "url"))
		snprintf(cf->url, sizeof(cf->url), "%s", value);
	else if (!strcmp(name, "cacert"))
		snprintf(cf->cacert, sizeof(cf->cacert), "%s", value);

	return 1;
}

int
load_config(struct cfg *config)
{
	int res;
	char fn[1024];
	struct passwd *pw;
	const char *homedir;

	pw = getpwuid(getuid());

	if (!pw)
	{
		perror("getpwuid");
		return 1;
	}
	homedir = pw->pw_dir;

	snprintf(fn, sizeof(fn), "%s/.synodl", homedir);
	res = ini_parse(fn, config_cb, config);

	if (res == -1)
	{
		perror(fn);
		return 1;
	}
	else if (res != 0)
	{
		fprintf(stderr, "Error parsing configuration file\n");
		return 1;
	}

	if (!strcmp(config->user, ""))
	{
		fprintf(stderr, "User name missing from configuration file\n");
		return 1;
	}
	if (!strcmp(config->pw, ""))
	{
		fprintf(stderr, "Password missing from configuration file\n");
		return 1;
	}
	if (!strcmp(config->url, ""))
	{
		fprintf(stderr, "URL missing from configuration file\n");
		return 1;
	}
	if (!strcmp(config->cacert, "ignore"))
	{
		config->verify_cert = 0;
	}

	return 0;
}
