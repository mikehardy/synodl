#include <math.h>
#include <stdio.h>
#include <string.h>

#include "ui-util.h"

int
selected_position(struct tasklist_ent *t, struct tasklist_ent *selected)
{
	int pos = 0;

	for (; t; t = t->next)
	{
		if (t == selected)
		{
			return pos;
		}

		pos += 1;
	}

	return -1;
}

void
print_size(size_t size, char *buf, size_t len)
{
	char u[] = "BkMGTPE";
	unsigned int cur, rem;

	cur = rem = 0;

	while ((size > 1000) && (cur < strlen(u)))
	{
		cur += 1;
		rem = size % 1000;
		size /= 1000;
	}

	if (size < 10)
	{
		snprintf(buf, len, "%1.1f%c",
				roundf(size * 1000 + rem) / 1000, u[cur]);
	}
	else
	{
		snprintf(buf, len, "%ld%c", lround(size), u[cur]);
	}
}
