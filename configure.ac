AC_INIT([synodl], 0.3.1)
AM_INIT_AUTOMAKE([foreign subdir-objects])
AC_PROG_CC

PKG_CHECK_MODULES([libjson], [json-c],
[
	AC_DEFINE([HAVE_JSON_C], [1], [Use JSON-C])
],[
	PKG_CHECK_MODULES([libjson], [json])
	AC_DEFINE([HAVE_JSON], [1], [Use JSON])
])
PKG_CHECK_MODULES([libcurl], libcurl)
PKG_CHECK_MODULES([libncursew], ncursesw)

# cunit: optional
PKG_CHECK_MODULES([CUNIT], [cunit],
[
	have_cunit="yes"
],
[
	AC_MSG_NOTICE([CUnit not found, unit tests won't work])
	have_cunit="no"
])

AM_CONDITIONAL([HAVE_CUNIT], [test x$have_cunit = xyes])

AC_CONFIG_HEADERS([config.h])
AC_CONFIG_FILES(Makefile src/Makefile)
AC_OUTPUT
