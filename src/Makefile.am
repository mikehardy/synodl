bin_PROGRAMS = synodl

EXTRA_DIST = cfg.h ini.h syno.h ui.h ui-util.h

synodl_SOURCES = synodl.c cfg.c syno.c ini.c ui.c ui-util.c
synodl_LDADD = @libjson_LIBS@ @libcurl_LIBS@ @libncursew_LIBS@ -lm
synodl_CFLAGS = @libjson_CFLAGS@ @libcurl_CFLAGS@ @libncursew_CFLAGS@

if HAVE_CUNIT
check_PROGRAMS = test_ui_util
TESTS = $(check_PROGRAMS)

test_ui_util_SOURCES = tests/ui-util.c ui-util.c
test_ui_util_LDADD = @CUNIT_LIBS@ -lm
test_ui_util_CFLAGS = @CUNIT_CFLAGS@
endif
