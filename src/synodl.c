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

#include <getopt.h>
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"
#include "cfg.h"
#include "syno.h"
#include "ui.h"

void help()
{
	printf("Syntax: synodl [options] [URL]\n\n");
	printf("If URL is empty a list of current download tasks is shown,\n");
	printf("otherwise the URL is added as a download task.\n\n");
	printf("  -h           Show this help\n");
	printf("\n");
	printf("This is %s.\n", PACKAGE_STRING);
	printf("Report bugs at https://code.ott.net/synodl/\n");
}

int main(int argc, char **argv)
{
	int c;
	const char *url;
	struct cfg config;
	struct session s;

	setlocale(LC_ALL, "");

	memset(&config, 0, sizeof(struct cfg));

	while (1)
	{
		c = getopt(argc, argv, "h");

		if (c < 0)
		{
			break;
		}

		switch (c)
		{
		case 'h':
			help();
			return EXIT_SUCCESS;
		default:
			help();
			return EXIT_FAILURE;
		}
	}

	if (load_config(&config) != 0)
	{
		fprintf(stderr, "Failed to load configuration\n");
		return EXIT_FAILURE;
	}

	memset(&s, 0, sizeof(struct session));

	if (syno_login(&config, &s) != 0)
	{
		return EXIT_FAILURE;
	}

	init_ui();

	if (optind < argc)
	{
		url = argv[optind];
		ui_add_task(&config, &s, url);
	}

	main_loop(&config, &s);

	syno_logout(&config, &s);
	free_ui();
	tasks_free();

	return EXIT_SUCCESS;
}
