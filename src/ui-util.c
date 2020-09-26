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
