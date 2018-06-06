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

#ifndef __SYNO_DL_CFG_H
#define __SYNO_DL_CFG_H

struct cfg
{
	char user[32];
	char pw[32];
	char url[64];
};

int load_config(struct cfg *config);
#endif
