#include <stdlib.h>
#include <unistd.h>

#include <CUnit/Basic.h>
#include <CUnit/Automated.h>

#include "ui-util.h"

static void
test_first_item_selected()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;
	t2.next = &t3;
	t3.prev = &t2;

	CU_ASSERT_EQUAL(selected_position(&t1, &t1), 0);
}

static void
test_middle_item_selected()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;
	t2.next = &t3;
	t3.prev = &t2;

	CU_ASSERT_EQUAL(selected_position(&t1, &t2), 1);
}

static void
test_last_item_selected()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;
	t2.next = &t3;
	t3.prev = &t2;

	CU_ASSERT_EQUAL(selected_position(&t1, &t3), 2);
}

static void
test_no_item_selected()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;
	t2.next = &t3;
	t3.prev = &t2;

	CU_ASSERT_EQUAL(selected_position(&t1, NULL), -1);
}

static void
test_invalid_item_selected()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;

	CU_ASSERT_EQUAL(selected_position(&t1, &t3), -1);
}

static void
test_selected_no_items()
{
	struct tasklist_ent t1, t2, t3;

	memset(&t1, 0, sizeof(struct tasklist_ent));
	memset(&t2, 0, sizeof(struct tasklist_ent));
	memset(&t3, 0, sizeof(struct tasklist_ent));

	t1.next = &t2;
	t2.prev = &t1;

	CU_ASSERT_EQUAL(selected_position(NULL, &t3), -1);
}

static int
test_selected_position()
{
	CU_pSuite suite;

	/* Add a suite to the registry */
	suite = CU_add_suite("Selected position", NULL, NULL);
	if (!suite)
	{
		return 1;
	}

	/* Add tests to the suite */
	if (
		!CU_add_test(suite, "First item selected",
			test_first_item_selected)
		|| !CU_add_test(suite, "Middle item selected",
			test_middle_item_selected)
		|| !CU_add_test(suite, "Last item selected",
			test_last_item_selected)
		|| !CU_add_test(suite, "No item selected",
			test_no_item_selected)
		|| !CU_add_test(suite, "Invalid item selected",
			test_invalid_item_selected)
		|| !CU_add_test(suite, "No items to select from",
			test_selected_no_items)
	)
	{
		return 1;
	}

	return 0;
}

static void
test_prefix_none()
{
	char buf[32];

	print_size(500, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "500B");
}

static void
test_prefix_decimal_point()
{
	char buf[32];

	print_size(7, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "7.0B");
}

static void
test_prefix_kb_decimal_point()
{
	char buf[32];

	print_size(5300, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "5.3k");
}

static void
test_prefix_kb()
{
	char buf[32];

	print_size(50000, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "50k");
}

static void
test_prefix_mb()
{
	char buf[32];

	print_size(50000000, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "50M");
}

static void
test_prefix_gb()
{
	char buf[32];

	print_size(50000000000, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "50G");
}

static void
test_prefix_tb()
{
	char buf[32];

	print_size(50000000000000, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "50T");
}

static void
test_prefix_pb()
{
	char buf[32];

	print_size(50000000000000000, buf, sizeof(buf));
	CU_ASSERT_STRING_EQUAL(buf, "50P");
}

static void
test_prefix_eb()
{
	char buf[32];

	print_size(4300000000000000000, buf, sizeof(buf));
	fprintf(stderr, "SIZE = %s\n", buf);
	CU_ASSERT_STRING_EQUAL(buf, "4.3E");
}

static int
test_size_prefix()
{
	CU_pSuite suite;

	/* Add a suite to the registry */
	suite = CU_add_suite("Size prefix", NULL, NULL);
	if (!suite)
	{
		return 1;
	}

	/* Add tests to the suite */
	if (
		!CU_add_test(suite, "No prefix for small sizes",
			test_prefix_none)
		|| !CU_add_test(suite, "Decimal point in very small sizes",
			test_prefix_decimal_point)
		|| !CU_add_test(suite, "Prefix for kB",
			test_prefix_kb)
		|| !CU_add_test(suite, "Prefix for kB with decimal point",
			test_prefix_kb_decimal_point)
		|| !CU_add_test(suite, "Prefix for MB",
			test_prefix_mb)
		|| !CU_add_test(suite, "Prefix for GB",
			test_prefix_gb)
		|| !CU_add_test(suite, "Prefix for TB",
			test_prefix_tb)
		|| !CU_add_test(suite, "Prefix for PB",
			test_prefix_pb)
		|| !CU_add_test(suite, "Prefix for EB",
			test_prefix_eb)
	)
	{
		return 1;
	}

	return 0;
}

static void
help()
{
}

int main(int argc, char **argv)
{
	int c, res;

	while ((c = getopt(argc, argv, "dh")) != -1)
	{
		switch(c)
		{
		case 'h':
			help();
			return EXIT_SUCCESS;
		default:
			return EXIT_FAILURE;
		}
	}

	/* Initialize the CUnit test registry */
	if (CU_initialize_registry() != CUE_SUCCESS)
	{
		return CU_get_error();
	}

	/* Add suites to the registry */
	if (
		test_selected_position()
		|| test_size_prefix()
	)
	{
		CU_cleanup_registry();
		return CU_get_error();
	}

	/* Run the tests */
	CU_basic_set_mode(CU_BRM_VERBOSE);
	CU_basic_run_tests();

	printf("\n");
	CU_basic_show_failures(CU_get_failure_list());
	printf("\n\n");

	CU_set_output_filename(argv[0]);
	CU_automated_run_tests();

	res = CU_get_number_of_failure_records();

	/* Clean up registry and return */
	CU_cleanup_registry();

	return res;
}
