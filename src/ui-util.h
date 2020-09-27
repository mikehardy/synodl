#ifndef __SYNODL_UI_UTIL_H
#define __SYNODL_UI_UTIL_H

#include <stdlib.h>

#include "syno.h"

struct tasklist_ent
{
	struct task *t;
	struct tasklist_ent *next;
	struct tasklist_ent *prev;
};

int selected_position(struct tasklist_ent *all, struct tasklist_ent *);
void print_size(size_t size, char *buf, size_t len);

#endif
