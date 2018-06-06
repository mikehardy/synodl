AC_INIT([synodl], 0.2.0)
AM_INIT_AUTOMAKE([foreign])
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

AC_CONFIG_HEADERS([config.h])
AC_CONFIG_FILES(Makefile src/Makefile)
AC_OUTPUT
