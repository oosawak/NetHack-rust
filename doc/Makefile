#
# This file is generated automatically.  Do not edit.
# Your changes will be lost.  See sys/unix/NewInstall.unx.
# Identify this file:
MAKEFILE_DOC=1

HINTSFILE=sys/unix/hints/linux.500

###
### Start sys/unix/hints/linux.500 PRE
###
# (new segment at source line 15 )
# linux.500 hints file provides a single-user build for Linux (such
# as Ubuntu focal).

# note: '#-INCLUDE' is not just a comment
#        multiw-1.500 contains sections 1 to 2

#------------------------------------------------------------------------------
# NetHack 5.0  multiw-1.500 $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $

# 1. Which windowing interface(s) should be included in this binary?
# One or more of these can be manually uncommented and/or can be specified
# on the 'make' command line.  If none are enabled, tty will be used.
#WANT_WIN_TTY=1
#WANT_WIN_CURSES=1
#WANT_WIN_X11=1
#WANT_WIN_QT=1

# 2. What is the default window system?
# Exactly one of these can be manually uncommented and/or can be specified
# on the 'make' command line.  If none is enabled, the first among
# WANT_WIN_{tty,curses,X11,Qt} that is enabled will become default.
#WANT_DEFAULT=tty
#WANT_DEFAULT=curses
#WANT_DEFAULT=Qt
#WANT_DEFAULT=X11

# Based on the NetHack version 5.0.0
# Used to build some filenames subsequently in other include files
NHV=500

#end of multiw-1.500
#------------------------------------------------------------------------------

HINTSVERSION := 500

ifdef MAKEFILE_TOP
ifeq ($(MAKELEVEL),0)
PRECHECK+=checkmakefiles
# all files included from this hints file get listed
# in HINTSINCLNAMES (without suffix and without a path)
HINTSINCLNAMES := compiler cross-pre1 cross-pre2 cross-post \
		gbdates-pre gbdates-post \
		multiw-1 multiw-2 misc \
		multisnd1-pre multisnd2-pre multisnd-post
HINTSINCLFILES := $(addsuffix .$(HINTSVERSION), $(HINTSINCLNAMES))
endif
endif

ifndef LIBXPM
LIBXPM= -L/opt/X11/lib -lXpm
endif

#4. Other
GAMEUID  = $(USER)
GAMEGRP  = games

# This gives better backtraces by making all core functions global; this
# works around a limitation in glibc's backtrace(3) function.
# This will be turned on automatically with CRASHREPROT.
# 1 to enable, 0 to disable
USE_NOSTATICFN = 1
# If you want CRASHREPORT but absolutely don't want NOSTATICFN, define this:
#USE_NONOSTATICFN = 1

#-----------------------------------------------------------------------------
#===============-=================================================
# NetHack 5.0  include/cross-pre1. $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $
#
# Cross-compiling -PRE section 1
#

ifdef CROSS_TO_MSDOS
CROSS=1
BUILD_TARGET_LUA=1
BUILD_PDCURSES=1
CROSS_SHARED=1
override TARGET = msdos
override TARGETDIR=../targets/$(TARGET)
override TARGETPFX = $(TARGETDIR)/
override TARGET_LIBS=
endif

ifdef CROSS_TO_WASM
CROSS=1
BUILD_TARGET_LUA=1
HACKDIR=/
PREFIX=
override TARGET = wasm
override TARGETDIR=../targets/$(TARGET)
override TARGETPFX = $(TARGETDIR)/
override TARGET_LIBS=
endif

ifdef CROSS_TO_MIPS
CROSS=1
BUILD_TARGET_LUA=1
BUILD_TARGET_NCURSES=1
HACKDIR=/
PREFIX=
override TARGET = mips
override TARGETDIR=../targets/$(TARGET)
override TARGETPFX = $(TARGETDIR)/
override TARGET_LIBS=
endif

ifdef CROSS_TO_AMIGA
CROSS=1
BUILD_TARGET_LUA=1
CROSS_SHARED=1
HACKDIR=NetHack:
override TARGET = amiga
override TARGETDIR=../targets/$(TARGET)
override TARGETPFX = $(TARGETDIR)/
override TARGET_LIBS=
endif

ifdef CROSS
override PREGAME=
override BUILDMORE=
override CLEANMORE=
override PACKAGE=
endif
# End of cross-compiling -PRE section 1
#===============-=================================================



#-----------------------------------------------------------------------------
# You shouldn't need to change anything below here (in the hints file; if
# you're reading this in Makefile augmented by hints, that may not be true).
#

#------------------------------------------------------------------------------
# NetHack 5.0  multiw-2.500 $NHDT-Date: 1599337709 2020/09/05 20:28:29 $  $NHDT-Branch: NetHack-5.0 $
#
# Sorts out support for multiple window ports (interfaces) to included in the build.
#
# Included from:
#           hints/linux.500
#           hints/macOS.500
#
# The following will be set appropriately following this:
#     - WANT_WIN_XXX  (at least one will be set; default is TTY)
#     - WANT_DEFAULT (set to match one of the enabled WANT_WIN_XXX)
#     - WINCFLAGS
#     - WINSRC
#     - WINOBJ0
#---
# User selections could be specified as combinations of any of the following:
# WANT_WIN_TTY=1, WANT_WIN_CURSES=1, WANT_WIN_QT=1, WANT_WIN_X11=1
# The selections will all be linked into the same binary.
#
# Assuming you have the prerequisite packages mentioned above, you can
# specify, right on the make command line, which window ports (or interfaces)
# to include in your build. Doing it via the make command line means that won't
# have to edit the Makefile.
#
# make WANT_WIN_QT=1 WANT_WIN_X11=1 WANT_WIN_CURSES=1 WANT_WIN_TTY=1 install
#
# Add WANT_DEFAULT=Qt (or other interface) if you want nethack to use
# something other than tty as the default interface.
#

ifdef WANT_WIN_ALL
WANT_WIN_TTY=1
WANT_WIN_CURSES=1
WANT_WIN_X11=1
WANT_WIN_QT=1
else
# Make sure that at least one interface is enabled.
ifndef WANT_WIN_TTY
ifndef  WANT_WIN_CURSES
ifndef   WANT_WIN_X11
ifndef    WANT_WIN_QT
WANT_WIN_TTY=1
endif
endif
endif
endif
endif

ifdef WANT_LIBNH
WANT_DEFAULT=shim
endif

# Make sure that a default interface is specified; this doesn't guarantee
# sanity for something like 'make WANT_WIN_CURSES=1 WANT_DEFAULT=X11' but
# 'makedefs -v' would notice, complain, and quit causing 'make' to quit.
ifndef WANT_DEFAULT
# pick the first one enabled among { tty, curses, X11, Qt }
ifdef WANT_WIN_TTY
WANT_DEFAULT=tty
else
ifdef  WANT_WIN_CURSES
WANT_DEFAULT=curses
else
ifdef   WANT_WIN_X11
WANT_DEFAULT=X11
else
ifdef    WANT_WIN_QT
WANT_DEFAULT=Qt
else
# ? shouldn't be able to get here...
endif
endif
endif
endif
endif

WINCFLAGS=
WINSRC =
WINOBJ0 =
SND_CFLAGS=
XTRASRC =
XTRAOBJ =

ifdef WANT_WIN_TTY
WINSRC += $(WINTTYSRC)
WINOBJ0 += $(WINTTYOBJ)
else
WINCFLAGS += -DNOTTYGRAPHICS
endif

ifdef WANT_WIN_CURSES
WINCFLAGS += -DCURSES_GRAPHICS
ifeq "$(USE_GENL_PUTMIXED)" "1"
WINCFLAGS += -DUSE_GENL_PUTMIXED
else
WINCFLAGS += -D_XOPEN_SOURCE_EXTENDED=1
endif
WINSRC += $(WINCURSESSRC)
WINOBJ0 += $(WINCURSESOBJ)
endif

ifdef WANT_WIN_X11
WINCFLAGS += -DX11_GRAPHICS
WINSRC += $(WIINX11SRC)
WINOBJ0 += $(WINX11OBJ)
XTRASRC += tile.c
XTRAOBJ += $(TARGETPFX)tile.o
endif

ifndef WANT_WIN_QT
ifdef WANT_WIN_ALL
WANT_WIN_QT=1
endif
ifdef WANT_WIN_QT4
ifndef WANT_WIN_QT
WANT_WIN_QT=1
endif   # not WANT_WIN_QT
endif   # WANT_WIN_QT4
ifdef WANT_WIN_QT5
ifndef WANT_WIN_QT
WANT_WIN_QT=1
endif   # not WANT_WIN_QT
endif   # WANT_WIN_QT5
ifdef WANT_WIN_QT6
ifndef WANT_WIN_QT
WANT_WIN_QT=1
endif   # not WANT_WIN_QT
endif   # WANT_WIN_QT6
endif   # not def WANT_WIN_QT

ifdef WANT_WIN_QT
# WANT_WIN_QT5 is the default
ifndef WANT_WIN_QT4
ifndef WANT_WIN_QT5
ifndef WANT_WIN_QT6
WANT_WIN_QT5=1
endif   # not WANT_WIN_QT6
endif   # not WANT_WIN_QT5
endif   # not WANT_WIN_QT4
ifdef WANT_WIN_QT4
#Slackware 14.2 puts Qt4 here
#if your Qt4 is elsewhere, change this to match
QT4DIR=/usr/lib64/qt
QTDIR=$(QT4DIR)
endif  # WANT_WIN_QT4
endif   # WANT_WIN_QT

ifdef WANT_WIN_QT
WINCFLAGS += -DQT_GRAPHICS
WINSRC += $(WINQTSRC)
WINOBJ0 += $(WINQTOBJ)
ifndef WANT_WIN_QT4
SNDCFLAGS += -DSND_LIB_QTSOUND
endif #! WANT_WIN_QT4
HAVE_SNDLIB = 1
NEEDS_SND_USERSOUNDS = 1
NEEDS_SND_SEAUTOMAP = 1
XTRASRC += tile.c
XTRAOBJ += $(TARGETPFX)tile.o
#
ifndef CPLUSPLUS_NEEDED
CPLUSPLUS_NEEDED = 1
endif   # CPLUSPLUS_NEEDED
ifdef WANT_WIN_QT6
CPLUSPLUS_NEED20 = 1
CPLUSPLUS_NEED_DEPSUPPRESS = 1
endif   # WANT_WIN_QT6
endif   # WANT_WIN_QT

ifeq "$(GIT)" "1"
ifndef GITSUBMODULES
GITSUBMODULES=1
endif
endif

ifeq "$(git)" "1"
ifndef GITSUBMODULES
GITSUBMODULES=1
endif
endif

ifeq "$(CPPREGEX)" "1"
REGEXOBJ=$(TARGETPFX)cppregex.o
ifndef CPLUSPLUS_NEEDED
CPLUSPLUS_NEEDED = 1
endif
endif

ifeq "$(cppregex)" "1"
REGEXOBJ=$(TARGETPFX)cppregex.o
ifndef CPLUSPLUS_NEEDED
CPLUSPLUS_NEEDED = 1
endif
endif

ifeq "$(musl)" "1"
MUSL=1
endif
ifeq "$(MUSL)" "1"
ifneq "$(NOCRASHREPORT)" "1"
NOCRASHREPORT=1
endif
WINCFLAGS += -DMUSL_LIBC
# use this instead of col -bx
COLCMD=../util/stripbs
STRIPBS=../util/stripbs
else
WINCFLAGS += -DGNU_LIBC
endif

ifeq "$(NOCRASHREPORT)" "1"
WINCFLAGS += -DNOCRASHREPORT
endif

#end of hints/include/multiw-2.500
#------------------------------------------------------------------------------


# compiler.500 contains compiler detection and adjustments common
# to both linux and macOS

#------------------------------------------------------------------------------
# NetHack 5.0  compiler.500 $NHDT-Date: 1668359835 2022/11/13 17:17:15 $  $NHDT-Branch: NetHack-5.0 $

# compiler flags:  CCFLAGS is used to construct a value for CFLAGS with
#	various -I, -D, and -W settings appended below;
#	these are the settings of most interest for an end-user build
#	(clang doesn't support '-Og', gcc needs 4.x or later)
CCFLAGS = -g
#CCFLAGS = -g -Og
#CCFLAGS = -O2
#	Note: this is not the usual 'CFLAGS' which is used in default
#	rules for compiling C code; specifying a value for that on the
#	'make' command line should be avoided.

#If you want to force use of one particular set of compilers, do it
#here, ahead of the detection, so that the detection will match your
#choice and set variables CCISCLANG, CCISGCC, GCCGTEQ, CLANGPPGTEQ9 etc.
#accordingly.
#
#CC= gcc
#CXX= g++ -std-gnu++11
#
#CC= clang
#CXX=clang++ -std=gnu++11
#
# If these are set on entry, preparation for C++ compiles is affected.
#    CPLUSPLUS_NEEDED = 1           C++ compile bits included
#    CPLUSPLUS_NEED17 = 1           C++ -std=c++17 (at least)
#    CPLUSPLUS_NEED20 = 1           C++ -std=c++20 (at least)
#    CPLUSPLUS_NEED_DEPSUPPRESS = 1 C++ -Wno-deprecated-copy,
#                                       -Wno-deprecated-declarations

ifeq "$(WANT_ASAN)" "1"
USE_ASAN=1
endif

ifeq "$(WANT_UBSAN)" "1"
USE_UBSAN=1
endif

# If you want to override the compiler detection just carried out
# uncomment one of the following pairs. Note, however, that
# doing this after the detection above will likely result in
# mismatched variable values for CCISCLANG, CCISGCC, GCCGTEQ, CLANGPPGTEQ9 etc.
#
#CC= gcc
#CXX= g++ -std-gnu++11
#
#CC= clang
#CXX=clang++ -std=gnu++11

CFLAGS=$(CCFLAGS) -I../include -DNOTPARMDECL
CFLAGS+=-Wall -Wextra  \
	-Wreturn-type -Wunused -Wformat -Wswitch -Wshadow -Wwrite-strings
CFLAGS+=-pedantic
CFLAGS+=-Wmissing-declarations
#CFLAGS+=-Wformat=2
#CFLAGS+=-Wdiscarded-qualifiers

# these are left out of the C++ flags
CFLAGS+=-Wformat-nonliteral
CFLAGS+=-Wunreachable-code

#
# the following are not allowed in C++
CFLAGS+=-Wimplicit
CFLAGS+=-Wimplicit-function-declaration
CFLAGS+=-Wimplicit-int
CFLAGS+=-Wmissing-prototypes
CFLAGS+=-Wold-style-definition
CFLAGS+=-Wstrict-prototypes
CFLAGS+=-Wnonnull

#detection of clang vs gcc
CCTEST := $(shell echo `$(CC) --version` | grep clang)
ifneq "$(CCTEST)" ""
CCISCLANG=1
CCISGCC=
else
CCISCLANG=
CCISGCC=1
endif
#$(info CCISCLANG=$(CCISCLANG), CCISGCC=$(CCISGCC))

ifeq "$(CCISGCC)" "1"
# gcc-specific follows
CXX=g++ -std=gnu++11
# get the version of gcc
GCCGTEQ9 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 9)
GCCGTEQ11 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 11)
GCCGTEQ12 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 12)
GCCGTEQ14 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 14)
ifeq "$(GCCGTEQ9)" "1"
# flags present in gcc version greater than or equal to 9 can go here
CFLAGS+=-Wformat-overflow
CFLAGS+=-Wmissing-parameter-type
endif   # GCC greater than or equal to 9
#ifeq "$(GCCGTEQ11)" "1"
CFLAGS+=-Wimplicit-fallthrough
#endif
#ifeq "$(GCCGTEQ12)" "1"
#endif
#ifeq "$(GCCGTEQ14)" "1"
#endif
# end of gcc-specific
else   # gcc or clang?
# clang-specific follows
CXX=clang++ -std=gnu++11
CLANGGTEQ12 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 12)
CLANGGTEQ14 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 14)
CLANGGTEQ21 := $(shell expr `$(CC) -dumpversion | cut -f1 -d.` \>= 21)
ifeq "$(CLANGGTEQ12)" "1"
CFLAGS+=-Wimplicit-fallthrough
endif
ifeq "$(CLANGGTEQ14)" "1"
ifneq "$(VIEWDEPRECATIONS)" "1"
CFLAGS+=-Wno-deprecated-declarations
endif  # not VIEWDEPRECATIONS
else
# older versions complain about things newer ones don't without this
CFLAGS+=-Wno-missing-field-initializers
endif
ifeq "$(CLANGGTEQ21)" "1"
CFLAGS+=-Wno-deprecated-octal-literals
endif
# none
endif  # clang-specific ends here

ifdef MAKEFILE_SRC
ifdef CPLUSPLUS_NEEDED
CCXXFLAGS = -g -I../include -DNOTPARMDECL
CCXXFLAGS+=-Wall -Wextra -Wno-missing-field-initializers \
        -Wreturn-type -Wunused -Wformat -Wswitch -Wshadow -Wwrite-strings
CCXXFLAGS+=-pedantic
CCXXFLAGS+=-Wmissing-declarations
#detection of clang++ vs g++
CXXISCLANG := $(shell echo `$(CXX) --version` | grep clang)
ifeq "$(CXXISCLANG)" ""
# g++-specific
CCXX=g++ -std=gnu++11
# get the version of g++
GPPGTEQ9 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 9)
GPPGTEQ11 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 11)
GPPGTEQ12 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 12)
GPPGTEQ14 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 14)
ifeq "$(GPPGTEQ9)" "1"
CCXXFLAGS+=-Wformat-overflow
ifdef CPLUSPLUS_NEED_DEPSUPPRESS
ifneq "$(VIEWDEPRECATIONS)" "1"
CCXXFLAGS+=-Wno-deprecated-copy
CCXXFLAGS+=-Wno-deprecated-declarations
endif  # not VIEWDEPRECATIONS
endif  # CPLUSPLUS_NEED_DEPSUPPRESS
endif  # g++ version greater than or equal to 9
ifeq "$(GPPGTEQ11)" "1"
# the g++ linker will have trouble linking if the following isn't included
# when compiling the C files.
CFLAGS+=-fPIC
endif  # g++ version greater than or equal to 11
ifdef CPLUSPLUS_NEED17
ifeq "$(GPPGTEQ12)" "1"
CCXX=g++ -std=c++17
else   # g++ version greater than or equal to 12? (no follows)
CCXX=g++ -std=c++17
endif  # g++ version greater than or equal to 12
endif  # CPLUSPLUS_NEED17
ifdef CPLUSPLUS_NEED20
ifeq "$(GPPGTEQ9)" "1"
CCXX=g++ -std=c++20
else   # g++ version greater than or equal to 9? (no follows)
CCXX=g++ -std=c++17
endif  # g++ version greater than or equal to 9
endif  # CPLUSPLUS_NEED20


else   # g++ or clang++ ?

# clang++-specific
CCXX=clang++ -std=c++11
# get the version of clang++
CLANGPPGTEQ9 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 9)
CLANGPPGTEQ11 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 11)
CLANGPPGTEQ14 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 14)
CLANGPPGTEQ16 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 16)
CLANGPPGTEQ17 := $(shell expr `$(CXX) -dumpversion | cut -f1 -d.` \>= 17)
ifeq "$(CLANGPPGTEQ9)" "1"
#CCXXFLAGS+=-Wformat-overflow
endif
ifeq "$(CLANGPPGTEQ14)" "1"
CPLUSPLUS_NEED_DEPSUPPRESS=1
endif
ifdef CPLUSPLUS_NEED_DEPSUPPRESS
ifneq "$(VIEWDEPRECATIONS)" "1"
CCXXFLAGS+=-Wno-deprecated
CCXXFLAGS+=-Wno-deprecated-declarations
endif  # not VIEWDEPRECATIONS
endif  # CPLUSPLUS_NEED_DEPSUPPRESS
# The clang++ linker seems to have trouble linking if the following isn't
# included when compiling the C files by clang..
CFLAGS+=-fPIC
ifdef CPLUSPLUS_NEED17
ifeq "$(CLANGPPGTEQ14)" "1"
CCXX=clang++ -std=c++17
endif  # clang++ greater than or equal to 14
ifeq "$(CLANGPPGTEQ17)" "1"
CCXX=clang++ -std=c++17
endif  # clang++ greater than or equal to 17
endif  # CPLUSPLUS_NEED17
ifdef CPLUSPLUS_NEED20
ifeq "$(CLANGPPGTEQ17)" "1"
CCXX=clang++ -std=c++20
endif  # clang++ greater than or equal to 17
endif  # CPLUSPLUS_NEED20
endif  # end of clang++-specific section
CXX=$(CCXX)
endif  # CPLUSPLUS_NEEDED
endif   # MAKEFILE_SRC

ifeq "$(c89)" "1"
HAVECSTD=c89
endif
ifeq "$(C89)" "1"
HAVECSTD=c89
endif

ifeq "$(c99)" "1"
HAVECSTD=c99
endif
ifeq "$(C99)" "1"
HAVECSTD=c99
endif

ifeq "$(c11)" "1"
HAVECSTD=c11
endif
ifeq "$(C11)" "1"
HAVECSTD=c11
endif

ifeq "$(c23)" "1"
HAVECSTD=c2x
endif
ifeq "$(C23)" "1"
HAVECSTD=c2x
endif

ifeq "$(c2x)" "1"
HAVECSTD=c2x
endif
ifeq "$(C2X)" "1"
HAVECSTD=c2x
endif

ifeq "$(c2y)" "1"
HAVECSTD=c2y
endif
ifeq "$(C2Y)" "1"
HAVECSTD=c2y
endif

ifneq "$(HAVECSTD)" ""
CSTD = -std=$(HAVECSTD)
endif

#end of compiler.500
#------------------------------------------------------------------------------

ifdef WANT_WIN_QT
ifdef WANT_WIN_QT5
QTDIR=/usr
endif  # WANT_WIN_QT5
ifdef WANT_WIN_QT6
#if your Qt6 is elsewhere, change this to match
QTDIR=/usr/local/qt6
ifeq "$(GPPGTEQ14)" "1"
CCXXFLAGS += -Wno-template-id-cdtor
endif    # g++ greater than or equal to 14
endif   # WANT_WIN_QT6
endif  # WANT_WIN_QT

# misc.500 must come after compiler.500
# and after QTDIR is defined.
#
#------------------------------------------------------------------------------
# NetHack 5.0  misc.500 $NHDT-Date: 1668359836 2022/11/13 17:17:16 $  $NHDT-Branch: NetHack-5.0 $
#
# Further set-up  for miscellaneous odds and ends (after compiler.500)
#
# Included from:
#           hints/linux.500
#           hints/macOS.500
#

# This ensures that .moc files are compatible with the version of Qt chosen.
#
ifdef MAKEFILE_SRC
ifdef WANT_WIN_QT
# when switching from Qt5 to Qt6 or vice versa, any old .moc files will be
# incompatible; get rid of them in case user hasn't run 'make spotless';
# object files are incompatible with Qt library, so get rid of them too;
# Qt*.h-t are empty timestamp files and at most one should exist
QTany_H = Qt*.h-t
ifdef WANT_WIN_QT6
# Qt 6 builds and runs (with a couple of warnings) but needs more testing
QTn_H = Qt6.h-t
else
# Qt 5 is currently the default version for nethack NetHack 5.0.x's Qt interface
QTn_H = Qt5.h-t
endif
$(QTn_H) ::
	@if test ! -f $@; then ( rm -f $(QTany_H) *.moc qt_*.o; touch $@ ); fi;
endif #WANT_WIN_QT
endif #MAKFILE_SRC

ifdef WANT_WIN_TTY
USE_CURSESLIB=1
endif

ifdef WANT_WIN_CURSES
ifneq "$(USE_CURSESLIB)" "1"
USE_CURSESLIB=1
endif
endif

ifeq "$(GIT)" "1"
ifndef GITSUBMODULES
GITSUBMODULES=1
endif
endif

ifeq "$(git)" "1"
ifndef GITSUBMODULES
GITSUBMODULES=1
endif
endif

ifeq "$(CPPREGEX)" "1"
REGEXOBJ=$(TARGETPFX)cppregex.o
ifndef CPLUSPLUS_NEEDED
CPLUSPLUS_NEEDED = 1
endif
endif

ifeq "$(cppregex)" "1"
REGEXOBJ=$(TARGETPFX)cppregex.o
ifndef CPLUSPLUS_NEEDED
CPLUSPLUS_NEEDED = 1
endif
endif

ifdef USE_MANDOC
NROFF = mandoc
MAN2TXTPRE = -T ascii
MAN2TXTPOST= | col -b
else
#
# Detect groff.
NROFFISGROFF := $(shell echo `nroff --version | grep -c 'GNU.*groff.*version'`)
#$(info NROFFISGROFF=$(NROFFISGROFF))
ifneq "$(NROFFISGROFF)" "0"
# Gather groff's minor version number (register `.y`).
GROFFMINORVERSION := $(shell printf '.tm \\n[.y]\n' | nroff 2>&1)
#$(info GROFFMINORVERSION=$(GROFFMINORVERSION))
# Silence warnings produced by tmac.n, which NetHack does not modify.
NROFF_FLAGS := -wall -Wrange -Wscale -Wtab
# groff <= 1.23 also supported an "el" warning category that was buggy.
GROFFLE123 := $(shell expr $(GROFFMINORVERSION) \<= 23)
#$(info GROFFLE123=$(GROFFLE123))
ifeq "$(GROFFLE123)" "1"
NROFF_FLAGS += -Wel
endif  # end groff less than 1.23
endif  # end NROFFISGROFF
# $(info NROFF_FLAGS=$(NROFF_FLAGS))

ifneq "$(NROFFISGROFF)" ""   # It's groff
# add the -Tascii flag used by groff
MAN2TXTPRE += -Tascii
# nroff in groff 1.23 supports the -P option to pass arguments to the
# output driver.  -cbou are flags to grotty(1).
GROFFGE123 := $(shell expr $(GROFFMINORVERSION) \>= 23)
#$(info GROFFGE123=$(GROFFGE123))
ifeq "$(GROFFGE123)" "1"
MAN2TXTPRE += -P -cbou
MAN2TXTPOST=
else
MAN2TXTPRE += -c
endif  # end groff less than 1.23
endif  # end groff-specific
endif  # not USE_MANDOC

#end of hints/include/misc.500
#------------------------------------------------------------------------------


ifeq "$(USE_ASAN)" "1"
CFLAGS+=-fsanitize=address
LFLAGS+=-fsanitize=address
endif

ifeq "$(USE_UBSAN)" "1"
CFLAGS+=-fsanitize=undefined
LFLAGS+=-fsanitize=undefined
endif

ifeq "$(USE_CURSESLIB)" "1"
# default
CURSESLIB = -lncurses -ltinfo
# If CURSES_UNICODE is defined, we need ncursesw.
# Without CURSES_UNICODE the following simpler setting works.
# CURSESLIB = -lncurses -ltinfo
ifdef MAKEFILE_SRC
comma:=,
NCURSES_LFLAGS = $(shell pkg-config ncursesw --libs)
NCURSES_CFLAGS = $(shell pkg-config ncursesw --cflags)
ifeq (,$(findstring ncursesw, $(NCURSES_LFLAGS)))
ifeq (,$(findstring ncurses, $(NCURSES_LFLAGS)))
#this indicates that pkg-config itself was unavailable
NCURSES_LFLAGS = -lncursesw -ltinfo
endif
endif
#$(info $(NCURSES_LFLAGS))
ifeq (,$(findstring ncursesw, $(NCURSES_LFLAGS)))
HAVE_NCURSESW=0
else
HAVE_NCURSESW=1
endif
#$(info $(NCURSES_LFLAGS))
#$(info HAVE_NCURSESW=$(HAVE_NCURSESW))
ifeq "$(HAVE_NCURSESW)" "1"
# remove unnecessary -Wl,-Bsymbolic-functions if present
ifneq (,$(findstring -Wl$(comma)-Bsymbolic-functions, $(NCURSES_LFLAGS)))
CURSESLIB = $(subst -Wl$(comma)-Bsymbolic-functions,,$(NCURSES_LFLAGS))
else
CURSESLIB = $(NCURSES_LFLAGS)
endif
endif      #HAVE_NCURSESW
#$(info $(CURSESLIB))
ifeq (,$(findstring ncursesw, $(NCURSES_LFLAGS)))
ifeq (,$(findstring ncurses, $(NCURSES_LFLAGS)))
#this indicates that pkg-config itself was unavailable
NOPKGCONFIG = 1
endif      #ncurses not in NCURSES_LFLAGS?
endif      #ncursesw not in NCURSES_LFLAGS?
#
ifeq "$(NOPKGCONFIG)" "1"
NCURSES_CFLAGS = -D_DEFAULT_SOURCE -D_XOPEN_SOURCE=600
ifeq "$(HAVE_NCURSESW)" "1"
ifeq (,$(wildcard /usr/include/ncursesw/curses.h))
NCURSES_CFLAGS += -I/usr/include
else
NCURSES_CFLAGS += -I/usr/include/ncursesw
endif
NCURSES_LFLAGS = -lncursesw -ltinfo
else       #HAVE_NCURSESW
ifeq (,$(wildcard /usr/include/ncurses/curses.h))
NCURSES_CFLAGS += -I/usr/include
else
NCURSES_CFLAGS += -I/usr/include/ncurses
endif
NCURSES_LFLAGS = -lncurses -ltinfo
endif      #HAVE_NCURSESW
endif      #NOPKGCONFIG
endif      #MAKEFILE_SRC
endif      #USE_CURSESLIB

# NetHack sources control
NHCFLAGS+=-DDLB
NHCFLAGS+=-DHACKDIR=\"$(HACKDIR)\"
NHCFLAGS+=-DDEFAULT_WINDOW_SYS=\"$(WANT_DEFAULT)\"
NHCFLAGS+=-DSYSCF -DSYSCF_FILE=\"$(HACKDIR)/sysconf\" -DSECURE
NHCFLAGS+=-DTIMED_DELAY
NHCFLAGS+=-DDUMPLOG
NHCFLAGS+=-DCONFIG_ERROR_SECURE=FALSE
#NHCFLAGS+=-DGREPPATH=\"/usr/bin/grep\"
NHCFLAGS+=-DCOMPRESS=\"/bin/gzip\" -DCOMPRESS_EXTENSION=\".gz\"
#NHCFLAGS+=-DNOMAIL
#NHCFLAGS+=-DEXTRA_SANITY_CHECKS
#NHCFLAGS+=-DEDIT_GETLIN
#NHCFLAGS+=-DSCORE_ON_BOTL
#NHCFLAGS+=-DMSGHANDLER
#NHCFLAGS+=-DTTY_TILES_ESCCODES
#NHCFLAGS+=-DTTY_SOUND_ESCCODES
#NHCFLAGS+=-DNO_CHRONICLE
#NHCFLAGS+=-DLIVELOG
#NHCFLAGS+=-DCHANGE_COLOR
NHCFLAGS+=-DSELF_RECOVER
ifdef WANT_WIN_CURSES
NHCFLAGS+=$(NCURSES_CFLAGS)
ifeq "$(HAVE_NCURSESW)" "1"
NHCFLAGS+=-DCURSES_UNICODE
else
ifdef MAKEFILE_SRC
$(info Attention: CURSES_UNICODE is not being defined without ncursesw)
endif   #MAKEFILE_SRC
endif   #HAVE_NCURSESW
endif   #WANT_WIN_CURSES

#
#------------------------------------------------------------------------------
# NetHack 5.0  multisnd1-pre.500 $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $
#
# If it's cross-platform it belongs in this file.
# If it's macOS-only, it belongs in macOS.500.
# If it's linux-only, it belongs in linux.500.
#

#ifdef WANT_TESTSOUND
#HAVE_SNDLIB = 1
#SNDCFLAGS+= -DSND_LIB_TESTSOUND
## NEEDS_SND_USERSOUNDS
## NEEDS_SND_SEAUTOMAP
## NEEDS_WAV
#SNDLIBSRC = ../sound/testsound/testsound.c
#SNDLIBOBJ = testsound.o
#LFLAGS += -lm
#endif   # WANT_TESTSOUND

ifeq "$(HAVE_SNDLIB)" "1"
ifeq "$(WANT_SPEECH)" "1"
SNDCFLAGS+= -DSND_SPEECH
endif   # WANT_SPEECH
endif   # HAVE_SNDLIB

# HAVE_SNDLIB or NEEDS_SND_USERSOUNDS or NEEDS_SND_SEAUTOMAP 
# would have to be set prior to this. Any of them could have
# been set just above, or they could have been set in
# multiw-2 for Qt, or they could have been set in macOS.500
# or linux.500 for something platform-specific.
# Regardless, if one of those is set, the related preprocessor
# defines need to be added to SNDCFLAGS here.
#
ifeq "$(HAVE_SNDLIB)" "1"
ifeq "$(NEEDS_SND_USERSOUNDS)" "1"
SNDCFLAGS+= -DUSER_SOUNDS
endif   # NEEDS_SND_USERSOUNDS
ifeq "$(NEEDS_SND_SEAUTOMAP)" "1"
SNDCFLAGS+= -DSND_SOUNDEFFECTS_AUTOMAP
endif   # NEEDS_SND_SEAUTOMAP
endif   # HAVE_SNDLIB

#
#------------------------------------------------------------------------------

#

ifeq "$(CCISCLANG)" "1"
# clang-specific starts
# clang-specific ends
else
# gcc-specific starts
LIBCFLAGS+=-DSIG_RET_TYPE=__sighandler_t
# gcc-specific ends
endif

# WINCFLAGS set from multiw-2.500
# SNDCFLAGS set from multisnd-pre.500
CFLAGS+= $(WINCFLAGS)
CFLAGS+= $(SNDCFLAGS)
CFLAGS+= $(NHCFLAGS)
CFLAGS+= $(LIBCFLAGS)

# WINCFLAGS set from multiw-2.500
# SNDCFLAGS set from multisnd-pre.500
CCXXFLAGS+= $(WINCFLAGS)
CCXXFLAGS+= $(SNDCFLAGS)
CCXXFLAGS+= $(NHCFLAGS)
CCXXFLAGS+= $(LIBCFLAGS)

VARDATND =
VARDATND0 =

#ifdef WANT_WIN_CHAIN
#HINTSRC=$(CHAINSRC)
#HINTOBJ=$(CHAINOBJ)
#endif # WANT_WIN_CHAIN

ifdef MAKEFILE_SRC
ifdef CURSESLIB
WINLIB += $(CURSESLIB)
endif   #CURSESLIB

ifneq "$(NO_NHUUID)" "1"
UUID_LIB_CHECK := $(shell echo `/sbin/ldconfig -p | grep libuuid`)
#$(info UUID_LIB_CHECK=$(UUID_LIB_CHECK))
ifneq "$(UUID_LIB_CHECK)" ""
UUID_LIB=1
else
UUID_LIB=0
endif
#$(info UUID_LIB=$(UUID_LIB))
#
# check for uuid development headers
UUID_HEADERS_CHECK := $(wildcard /usr/include/uuid/uuid.h)
#$(info UUID_HEADERS_CHECK=$(UUID_HEADERS_CHECK))
ifneq "$(UUID_HEADERS_CHECK)" ""
UUID_HEADERS = 1
else
UUID_HEADERS = 0
endif
#$(info UUID_HEADERS=$(UUID_HEADERS))

ifeq "$(UUID_LIB)" "1"
ifeq "$(UUID_HEADERS)" "1"
ifndef WANT_NHUUID
WANT_NHUUID=1
endif
endif
endif
else    # NO_NHUUID
WANT_NHUUID=0
endif   # NO_NHUUID

ifeq "$(WANT_NHUUID)" "1"
#$(info NHUUID support will be included)
CFLAGS+= -DNHUUID
LIBS+=-luuid
else    # WANT_NHUUID
$(info Attention - No NHUUID support will be included)
endif   # WANT_NHUUID
endif   # MAKEFILE_SRC

ifdef WANT_WIN_X11
USE_XPM=1
WINX11LIB = -lXaw -lXmu -lXext -lXt -lX11
VARDATND0 += x11tiles NetHack.ad pet_mark.xbm pilemark.xbm
# -x: if built without dlb, some versions of mkfontdir think *.lev are fonts
POSTINSTALL += bdftopcf win/X11/nh10.bdf > $(HACKDIR)/nh10.pcf; ( cd $(HACKDIR); mkfontdir -x .lev );
# separate from CFLAGS so that we don't pass it to every file
X11CFLAGS = -I/opt/X11/include
# avoid repeated complaints about _X_NONNULL(args...) in <X11/Xfuncproto.h>
X11CFLAGS += -Wno-variadic-macros
ifdef USE_XPM
CFLAGS += -DUSE_XPM
WINX11LIB += -lXpm
VARDATND0 += rip.xpm
endif
WINLIB += $(WINX11LIB)
LFLAGS+=-L/opt/X11/lib
endif	# WANT_WIN_X11

ifdef WANT_WIN_QT
LINK = $(CXX)
ifdef WANT_WIN_QT4
QTCXXFLAGS += $(sort $(shell PKG_CONFIG_PATH=$(QTDIR)/lib/pkgconfig pkg-config QtGui --cflags)) -DQT_NO_SOUND=1
WINLIB += $(shell PKG_CONFIG_PATH=$(QTDIR)/lib/pkgconfig pkg-config QtGui --libs)
endif    # WANT_WIN_QT4
ifdef WANT_WIN_QT5
QTCXXFLAGS += $(sort $(shell PKG_CONFIG_PATH=$(QTDIR)/lib/pkgconfig pkg-config Qt5Gui Qt5Widgets Qt5Multimedia --cflags))
WINLIB += $(shell PKG_CONFIG_PATH=$(QTDIR)/lib/pkgconfig pkg-config Qt5Gui Qt5Widgets Qt5Multimedia --libs)
endif    # WANT_WIN_QT5
ifdef WANT_WIN_QT6
ifdef QT6MANUAL
# Try some likely spots for a self-built Qt6.
# You'll have to change these manually before using the hints file
# if they don't match the installed location on your system.
ifneq "$(HOSTTYPE)" ""
QTHOST=$(HOSTTYPE)
else
QTHOST=x86_64
endif
QTTOP=/usr/include/$(HOSTTYPE)-linux-gnu/qt6
ifneq ($(wildcard $(QTTOP)/*),)
#we don't set QTLOCATED=1 for this
QTINCDIR=/usr/include/$(QTHOST)-linux-gnu/qt6
QTLIBDIR=/usr/lib/$(QTHOST)-linux.gnu
QTCXXFLAGS += -DQT_WIDGETS_LIB -DQT_MULTIMEDIA_LIB -DQT_NETWORK_LIB -DQT_GUI_LIB -DQT_CORE_LIB
QTCXXFLAGS += -I$(QTINCDIR)/QtWidgets -I$(QTINCDIR) -I$(QTINCDIR)/QtMultimedia
QTCXXFLAGS += -I$(QTINCDIR)/QtNetwork -I$(QTINCDIR)/QtGui -I$(QTINCDIR)/QtCore
WINLIB += -L$(QTLIBDIR) -lQt6Widgets -lQt6Multimedia -lQt6Network -lQt6Gui -lQt6Core
MOCPATH = /usr/lib/qt6/libexec/moc
#/usr/lib/x86_64-linux-gnu/libQt6EglFSDeviceIntegration.so
endif    # QTTOP/*
ifndef QTINCDIR
ifneq ($(wildcard /usr/local/Qt6/*),)
QTDIR=/usr/local/Qt6
QTLOCATED=1
endif    # wildcard /usr/local/Qt6
ifneq ($(wildcard /usr/local/qt6/*),)
QTDIR=/usr/local/qt6
QTLOCATED=1
endif    # wildcard /usr/local/qt6
endif    # !QTINCDIR
ifdef QTLOCATED
QTCXXFLAGS += -I$(QTDIR)/include/QtCore
QTCXXFLAGS += -I$(QTDIR)/include/QtGui
QTCXXFLAGS += -I$(QTDIR)/include/QtMultimedia
QTCXXFLAGS += -I$(QTDIR)/include/QtWidgets
MOCPATH = $(QTDIR)/libexec/moc
WINLIB += -L$(QTDIR)/lib -lQt6Widgets -lQt6Multimedia -lQt6Network -lQt6Gui -lQt6Core
endif   # QTLOCATED
else    # !QT6MANUAL
MOCDIR = $(shell pkg-config Qt6Gui --variable=libexecdir)
ifneq ($(wildcard $(MOCDIR)/*),)
MOCPATH = $(MOCDIR)/moc
else
MOCPATH = /usr/lib/qt6/libexec/moc
endif   # MOCDIR
QTCXXFLAGS += $(sort $(shell pkg-config Qt6Gui Qt6Widgets Qt6Multimedia --cflags))
WINLIB += $(shell pkg-config Qt6Gui Qt6Widgets Qt6Multimedia --libs)
endif   # QT6MANUAL
endif   # WANT_WIN_QT6
ifndef QTDIR
$(error QTDIR not defined in the environment or Makefile)
endif	# QTDIR
# XXX if /Developer/qt exists and QTDIR not set, use that
# XXX make sure QTDIR points to something reasonable
QTCXXFLAGS += -fPIC
POSTINSTALL+= bdftopcf win/X11/nh10.bdf > $(INSTDIR)/nh10.pcf; \
		( cd $(INSTDIR); mkfontdir -x .lev );
VARDATND0 += nhtiles.bmp rip.xpm nhsplash.xpm
else    # WANT_WIN_QT
LINK = $(CC)
endif	# !WANT_WIN_QT

# prevent duplicate tile.o in WINOBJ
WINOBJ = $(WINOBJ0) $(sort $(XTRAOBJ))
# prevent duplicates in VARDATND if both X11 and Qt are being supported
VARDATND += $(sort $(VARDATND0))

GIT_HASH := $(shell echo `git rev-parse --verify HEAD` 2>&1)
GIT_BRANCH := $(shell echo `git rev-parse --abbrev-ref HEAD` 2>&1)
GIT_PREFIX := $(shell echo `git config nethack.substprefix` 2>&1)

ifdef GIT_HASH
GITHASH = -DNETHACK_GIT_SHA=\"$(GIT_HASH)\"
endif
ifdef GIT_BRANCH
GITBRANCH = -DNETHACK_GIT_BRANCH=\"$(GIT_BRANCH)\"
endif
ifdef GIT_PREFIX
GITPREFIX = -DNETHACK_GIT_PREFIX=\"$(GIT_PREFIX)\"
endif

ifdef WANT_LIBNH
CFLAGS += -DSHIM_GRAPHICS -DNOTTYGRAPHICS -DNOSHELL -DLIBNH -fpic
LIBNHSYSSRC = ../sys/libnh/libnhmain.c \
		../sys/share/ioctl.c ../sys/share/unixtty.c \
		../sys/unix/unixunix.c ../sys/unix/unixres.c \
		../win/shim/winshim.c
LIBNHSYSOBJ = $(TARGETPFX)libnhmain.o $(TARGETPFX)ioctl.o \
		$(TARGETPFX)unixtty.o $(TARGETPFX)unixunix.o \
		$(TARGETPFX)unixres.o $(TARGETPFX)winshim.o \
		$(TARGETPFX)date.o
#don't bother building the game executable as it will fail
#without winshim
override GAME=
MOREALL += ( cd src ; $(MAKE) pregame ; $(MAKE) $(TARGETPFX)libnh.a )
endif  # WANT_LIBNH

SYSCONFCREATE = cp sys/unix/sysconf $(INSTDIR)/sysconf
SYSCONFINSTALL = $(SYSCONFCREATE) && \
	 $(CHOWN) $(GAMEUID) $(INSTDIR)/sysconf && \
	 $(CHGRP) $(GAMEGRP) $(INSTDIR)/sysconf && \
	 chmod $(VARFILEPERM) $(INSTDIR)/sysconf;
SYSCONFENSURE = (if ! test -f $(INSTDIR)/sysconf ; then \
	 $(SYSCONFCREATE) && \
	 $(CHOWN) $(GAMEUID) $(INSTDIR)/sysconf && \
	 $(CHGRP) $(GAMEGRP) $(INSTDIR)/sysconf && \
	 chmod $(VARFILEPERM) $(INSTDIR)/sysconf; fi );

ifdef WANT_SOURCE_INSTALL
PREFIX=$(abspath $(NHSROOT))
#SHELLDIR=
HACKDIR=$(PREFIX)/playground
GAMEPERM = 0700
VARFILEPERM = 0600
VARDIRPERM = 0700
CFLAGS+=-DSYSCF -DSYSCF_FILE=\"$(HACKDIR)/sysconf\" -DSECURE
MOREALL=$(MAKE) install
else #!WANT_SOURCE_INSTALL
#PREFIX=/usr
PREFIX=$(wildcard ~)/nh/install
HACKDIR=$(PREFIX)/games/lib/nethackdir
SHELLDIR = $(PREFIX)/games
VARDIRPERM = 0755
VARFILEPERM = 0600
GAMEPERM = 0755
endif #?WANT_SOURCE_INSTALL

INSTDIR=$(HACKDIR)
VARDIR = $(HACKDIR)

ifdef MAKEFILE_TOP
TESTGDBPATH=/usr/bin/gdb
POSTINSTALL+= test -f $(TESTGDBPATH) || \
		sed -i -e 's;^GDBPATH=/usr/bin/gdb;\#GDBPATH=/usr/bin/gdb;' \
		-e 's;PANICTRACE_GDB=1;PANICTRACE_GDB=0;' $(INSTDIR)/sysconf;
POSTUPDATE+= test -f $(TESTGDBPATH) || \
		sed -i -e 's;^GDBPATH=/usr/bin/gdb;\#GDBPATH=/usr/bin/gdb;' \
		-e 's;PANICTRACE_GDB=1;PANICTRACE_GDB=0;' $(INSTDIR)/sysconf;
endif  #MAKEFILE_TOP

ifeq '$(USE_NONOSTATICFN)' '1'
CFLAGS += -DNONOSTATICFN
else
ifeq '$(USE_NOSTATICFN)' '1'
CFLAGS += -DNOSTATICFN
endif
endif

# Lua
# when building liblua.a, avoid warning that use of tmpnam() should be
# replaced by mkstemp(); the lua code doesn't use nethack's config.h so
# this needs to be passed via make rather than defined in unixconf.h
SYSCFLAGS=-DLUA_USE_LINUX
ifdef GITSUBMODULES
LUAFLAGS=CC='$(CC)' SYSCFLAGS='$(SYSCFLAGS)'
ifeq "$(CCISCLANG)" "1"
# clang
LUAFLAGS +=CWARNGCC=''
endif   # clang
override LUAHEADERS = submodules/lua
override LUA2NHTOP = ../..
override LUAMAKEFLAGS=$(LUAFLAGS)
endif   # GITSUBMODULES
DLLIB = -ldl

# Only needed for GLIBC stack trace:
LFLAGS+=-rdynamic

# if TTY_TILES_ESCCODES
#WINSRC += tile.c
#WINOBJ += tile.o
# endif

CHOWN=true
CHGRP=true

# manpages directory
MANDIR=/usr/share/man/man6
#
#===============-=================================================
# NetHack 5.0  include/cross-pre2 $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $
#
# Cross-compiling -PRE section 2
#

ifdef BUILD_TARGET_LUA
#===============-=================================================
# LUA library
# Source from http://www.lua.org/ftp/lua-5.4.8.tar.gz
#=================================================================
LUA_VERSION ?=5.4.8
LUATOP ?= ../lib/lua-$(LUA_VERSION)
LUASRCDIR ?= $(LUATOP)/src
LUAOBJFILES1 =  $(TARGETPFX)lapi.o $(TARGETPFX)lauxlib.o \
		$(TARGETPFX)lbaselib.o $(TARGETPFX)lcode.o \
		$(TARGETPFX)lcorolib.o $(TARGETPFX)lctype.o \
		$(TARGETPFX)ldblib.o
ifeq "$(LUA_VERSION)" "5.3.5"
LUAOBJFILES1 += $(TARGETPFX)lbitlib.o
endif
LUAOBJFILES2 =  $(TARGETPFX)ldebug.o $(TARGETPFX)ldo.o $(TARGETPFX)ldump.o \
		$(TARGETPFX)lfunc.o $(TARGETPFX)lgc.o $(TARGETPFX)linit.o \
		$(TARGETPFX)liolib.o $(TARGETPFX)llex.o
LUAOBJFILES3 =  $(TARGETPFX)lmathlib.o $(TARGETPFX)lmem.o \
		$(TARGETPFX)loadlib.o $(TARGETPFX)lobject.o \
		$(TARGETPFX)lopcodes.o $(TARGETPFX)loslib.o \
		$(TARGETPFX)lparser.o $(TARGETPFX)lstate.o
LUAOBJFILES4 =  $(TARGETPFX)lstring.o $(TARGETPFX)lstrlib.o \
		$(TARGETPFX)ltable.o $(TARGETPFX)ltablib.o \
		$(TARGETPFX)ltm.o $(TARGETPFX)lundump.o \
		$(TARGETPFX)lutf8lib.o $(TARGETPFX)lvm.o $(TARGETPFX)lzio.o
LUALIBOBJS = $(LUAOBJFILES1) $(LUAOBJFILES2) $(LUAOBJFILES3) $(LUAOBJFILES4)
LUACROSSLIB = $(TARGETPFX)lua$(subst .,,$(LUA_VERSION)).a
LUAINCL  = -I$(LUASRCDIR)
override BUILDMORE += $(LUACROSSLIB)
override CLEANMORE += rm -f $(LUACROSSLIB) ;
override TARGET_LIBS += $(LUACROSSLIB)
LIBLM = -lm
else
LUAINCL=
endif  # BUILD_TARGET_LUA

ifdef BUILD_PDCURSES
#===============-=================================================
# PD Curses library
#===============-=================================================
ifdef WANT_WIN_CURSES
PDCTOP = ../lib/pdcursesmod
ifdef WANT_DOSVGA
PDCPORT = $(PDCTOP)/dosvga
PDCURSESDEF= -I$(PDCTOP) -I$(PDCPORT) \
		-D"CURSES_GRAPHICS" -D"CURSES_BRIEF_INCLUDE" \
		-D"PDC_WIDE" -D"PDC_RGB" -D"CURSES_UNICODE"
else
PDCPORT = $(PDCTOP)/dos
PDCURSESDEF= -I$(PDCTOP) -I$(PDCPORT) \
		-D"CURSES_GRAPHICS" -D"CURSES_BRIEF_INCLUDE"
endif # WANT_DOSVGA
PDCLIBOBJ1= $(TARGETPFX)addch.o $(TARGETPFX)addchstr.o \
		$(TARGETPFX)addstr.o $(TARGETPFX)attr.o \
		$(TARGETPFX)beep.o $(TARGETPFX)bkgd.o \
		$(TARGETPFX)border.o $(TARGETPFX)clear.o \
		$(TARGETPFX)color.o $(TARGETPFX)delch.o \
		$(TARGETPFX)deleteln.o $(TARGETPFX)getch.o \
		$(TARGETPFX)getstr.o $(TARGETPFX)getyx.o \
		$(TARGETPFX)inch.o
PDCLIBOBJ2= $(TARGETPFX)inchstr.o $(TARGETPFX)initscr.o \
		$(TARGETPFX)inopts.o $(TARGETPFX)insch.o \
		$(TARGETPFX)insstr.o $(TARGETPFX)instr.o \
		$(TARGETPFX)kernel.o $(TARGETPFX)keyname.o \
		$(TARGETPFX)mouse.o $(TARGETPFX)move.o \
		$(TARGETPFX)outopts.o $(TARGETPFX)overlay.o
PDCLIBOBJ3= $(TARGETPFX)pad.o $(TARGETPFX)panel.o $(TARGETPFX)printw.o \
		$(TARGETPFX)refresh.o $(TARGETPFX)scanw.o \
		$(TARGETPFX)scr_dump.o $(TARGETPFX)scroll.o \
		$(TARGETPFX)slk.o $(TARGETPFX)termattr.o
PDCLIBOBJ4= $(TARGETPFX)touch.o $(TARGETPFX)util.o $(TARGETPFX)window.o \
		$(TARGETPFX)debug.o
PDCLIBOBJS = $(PDCLIBOBJ1) $(PDCLIBOBJ2) $(PDCLIBOBJ3) $(PDCLIBOBJ4)
PDCLIB = $(TARGETPFX)pdclib.a
PDCINCL = -I$(PDCTOP) -I$(PDCTOP)/pdcurses
PDCOBJS = $(TARGETPFX)pdcclip.o $(TARGETPFX)pdcdisp.o \
		$(TARGETPFX)pdcgetsc.o $(TARGETPFX)pdckbd.o \
		$(TARGETPFX)pdcscrn.o $(TARGETPFX)pdcsetsc.o \
		$(TARGETPFX)pdcutil.o
override TARGET_LIBS += $(PDCLIB)
override BUILDMORE += $(PDCLIB)
override CLEANMORE += rm -f $(PDCLIB) ;
else   #WANT_WIN_CURSES
PDCURSESDEF=
PDCLIBOBJS=
PDCOBJS=
PDCLIB=
PDCINCL=
endif  # WANT_WIN_CURSES
endif  # BUILD_PDCURSES

ifdef BUILD_TARGET_NCURSES
#=================================================================
# ncurses
# Source from https://invisible-island.net/datafiles/release/ncurses.tar.gz
#
#=================================================================
NCURSESLIBDIR ?= $(TARGETPFX)ncurses/lib
NCURSESLIB ?= $(NCURSESLIBDIR)/libncurses.a
#override TARGET_LIBS += $(NCURSESLIB)
override BUILDMORE += $(NCURSESLIB)
endif  # BUILD_TARGET_NCURSES

ifdef CROSS_TO_MSDOS
#=================================================================
# MSDOS cross-compile recipe
#=================================================================
# Uses an MSDOS djgpp cross-compiler on linux or macos.
#
# 1. You can obtain the cross-compiler for your system via:
#     sys/msdos/fetch-cross.sh
# 2. Then
#     make CROSS_TO_MSDOS=1 WANT_WIN_TTY=1 WANT_WIN_CURSES=1 all
#
# Source from http://www.lua.org/ftp/lua-5.4.8.tar.gz
#=================================================================

CFLAGS += -DCROSSCOMPILE

#
# Override the build tools and some obj files to
# reflect the msdos djgpp cross-compiler.
#
TOOLTOP1 = ../lib/djgpp/bin
TOOLTOP2 = ../lib/djgpp/i586-pc-msdosdjgpp/bin
override TARGET_CC = $(TOOLTOP1)/i586-pc-msdosdjgpp-gcc
override TARGET_CXX = $(TOOLTOP1)/i586-pc-msdosdjgpp-g++
override TARGET_AR = $(TOOLTOP1)/i586-pc-msdosdjgpp-gcc-ar
override TARGET_STUBEDIT = ../lib/djgpp/i586-pc-msdosdjgpp/bin/stubedit
ifdef MAKEFILE_SRC
FLDR=../
endif
ifdef MAKEFILE_TOP
FLDR=
endif
dostargetexes=$(FLDR)lib/djgpp/target/bin/
#
ifdef DOSBOX
dosbox=$(DOSBOX)
endif
ifdef dosbox
dosboxgameid=NH$(NHV)
dosboxtop=$(dosbox)
dosboxnhfolder=$(dosboxtop)/$(dosboxgameid)
dosboxnhsrc=$(dosboxnhfolder)/NHSRC
dosboxconfigfile=NETHACK.CNF
dosgdburl=http://www.mirrorservice.org/sites/ftp.delorie.com/pub/djgpp/current/v2gnu/gdb801b.zip
WANT_DEBUG=1
DEPLOY=deploytodosbox
endif  # dosbox
#
ifeq "$(WANT_DEBUG)" "1"
DBGFLAGS = -g
else
DBGFLAGS =
endif
MSDOS_GCC_CFLAGS = -Wall -Wextra -Wreturn-type -Wunused -Wformat \
	-Wswitch -Wshadow -Wwrite-strings -Wmissing-declarations \
	-Wunreachable-code \
	-Wimplicit -Wimplicit-function-declaration \
	-Wimplicit-int -Wmissing-prototypes -Wold-style-definition \
	-Wstrict-prototypes -Wnonnull -Wformat-overflow \
	-Wmissing-parameter-type -Wimplicit-fallthrough
#-Wno-missing-field-initializers -Wno-cast-function-type
#-Wno-format
MSDOS_PEDANTIC = -pedantic
MSDOS_GPP_CFLAGS = -Wall -Wextra -Wno-missing-field-initializers -Wreturn-type \
	-Wunused -Wformat -Wswitch -Wshadow -Wwrite-strings -pedantic \
	-Wmissing-declarations -Wformat-nonliteral -Wunreachable-code \
	-Wno-maybe-uninitialized
MSDOS_TARGET_CFLAGS = -c -O $(DBGFLAGS) -I../include -I../sys/msdos -I../win/share \
	$(LUAINCL) -DDLB $(PDCURSESDEF) -DTILES_IN_GLYPHMAP \
	-DCROSSCOMPILE -DCROSSCOMPILE_TARGET -DCROSS_TO_MSDOS \
	-D_NAIVE_DOS_REGS \
	$(MSDOS_GCC_CFLAGS) $(SNDCFLAGS)
MSDOS_TARGET_CXXFLAGS = -c -O $(DBGFLAGS) -I../include -I../sys/msdos -I../win/share \
        $(LUAINCL) -DDLB $(PDCURSESDEF) \
        -DUSE_TILES -DCROSSCOMPILE -DCROSSCOMPILE_TARGET -DCROSS_TO_MSDOS \
	-D_NAIVE_DOS_REGS \
	$(MSDOS_GPP_CFLAGS) $(SNDCFLAGS)
PDCINCL += -I$(PDCPORT)
PDC_TARGET_CFLAGS = $(MSDOS_TARGET_CFLAGS) -Wno-unused-parameter \
			-Wno-missing-prototypes
ifndef SKIP_FONTS_IN_CI
FONTVER = terminus-font-4.49.1
FONTTOP = ../lib/$(FONTVER)
DOSFONT = ../sys/msdos/fonts
FONTTARGETS = $(DOSFONT)/ter-u16b.psf $(DOSFONT)/ter-u16v.psf \
		$(DOSFONT)/ter-u18b.psf $(DOSFONT)/ter-u20b.psf \
		$(DOSFONT)/ter-u22b.psf $(DOSFONT)/ter-u24b.psf \
		$(DOSFONT)/ter-u28b.psf $(DOSFONT)/ter-u32b.psf
else
FONTTARGETS=
endif
LUABIN = ../lib/lua-$(LUA_VERSION)/src/lua
LUA_TARGET_CFLAGS = $(MSDOS_TARGET_CFLAGS)
override TARGET_CFLAGS = $(MSDOS_TARGET_CFLAGS) $(MSDOS_PEDANTIC)
override TARGET_CXXFLAGS = $(MSDOS_TARGET_CXXFLAGS)
ifdef CPLUSPLUS_NEEDED
override TARGET_LINK = $(TOOLTOP1)/i586-pc-msdosdjgpp-g++
else
override TARGET_LINK = $(TOOLTOP1)/i586-pc-msdosdjgpp-gcc
endif
override TARGET_LFLAGS=
override TARGET_LIBS += -lpc $(LIBLM)
override SYSSRC = ../sys/share/pcmain.c ../sys/msdos/msdos.c \
		../sys/share/pcsys.c ../sys/share/pctty.c \
		../sys/share/pcunix.c ../sys/msdos/video.c \
		../sys/msdos/vidtxt.c ../sys/msdos/pckeys.c \
		../sys/msdos/vidvga.c ../sys/msdos/vidvesa.c \
		../sys/msdos/font.c \
		../win/share/bmptiles.c ../win/share/giftiles.c \
		../win/share/tileset.c
override SYSOBJ= $(TARGETPFX)pcmain.o $(TARGETPFX)msdos.o \
		$(TARGETPFX)pcsys.o $(TARGETPFX)pctty.o \
		$(TARGETPFX)pcunix.o $(TARGETPFX)video.o \
		$(TARGETPFX)vidtxt.o $(TARGETPFX)pckeys.o \
		$(TARGETPFX)vidvga.o $(TARGETPFX)vidvesa.o \
		$(TARGETPFX)font.o \
		$(TARGETPFX)bmptiles.o $(TARGETPFX)giftiles.o \
		$(TARGETPFX)tileset.o $(TARGETPFX)tile.o
override WINLIB=
override LUALIB=
override LUALIBS=
override TOPLUALIB=
override GAMEBIN = $(TARGETPFX)nethack.exe
override RECOVERBIN = $(TARGETPFX)recover.exe
override SFCTOOLBIN = $(TARGETPFX)sfctool.exe
override PACKAGE = dospkg
override PREGAME += mkdir -p $(TARGETDIR) ; make $(TARGETPFX)exceptn.o ;
override CLEANMORE += rm -f -r $(TARGETDIR) ; rm -f -r $(FONTTARGETS) ;
ifeq "$(WANT_DEBUG)" "1"
GDBEXE=$(dostargetexes)gdb.exe
GDBBAT=NHGDB.BAT
GDBCMDLINE=directory nhsrc/src nhsrc/include nhsrc/sys/msdos \
		nhsrc/sys/share nhsrc/win/curses \
		nhsrc/win/tty
else
GDBEXE=
GDBBAT=
GDBCMDLINE=
endif
VARDATND += nhtiles.bmp
#
ifdef WANT_WIN_CURSES
# rules for pdcurses dos-specific files
$(TARGETPFX)%.o : $(PDCPORT)/%.c
	$(TARGET_CC) $(PDCINCL) $(PDC_TARGET_CFLAGS) -o$@  $<
endif  # WANT_WIN_CURSES
#
# Rule for files in sys/msdos
$(TARGETPFX)%.o : ../sys/msdos/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -o$@ $<
endif  # CROSS_TO_MSDOS
#=================================================================


ifdef CROSS_TO_WASM
#===============-=================================================
# WASM
# originally from https://github.com/NetHack/NetHack/pull/385
#===============-=================================================
#
#WASM_DEBUG = 1
WASM_DATA_DIR = $(TARGETPFX)wasm-data
WASM_TARGET = $(TARGETPFX)nethack.js
EMCC_LFLAGS =
#EMCC_LFLAGS += -s SINGLE_FILE=1
EMCC_LFLAGS += -DHACKDIR=\"$(HACKDIR)\"
EMCC_LFLAGS += -s WASM=1
EMCC_LFLAGS += -s ALLOW_TABLE_GROWTH
EMCC_LFLAGS += -s ASYNCIFY -s ASYNCIFY_IMPORTS='["local_callback"]'
EMCC_LFLAGS += -O3
EMCC_LFLAGS += -s MODULARIZE
EMCC_LFLAGS += -s EXPORTED_FUNCTIONS='["_main", "_shim_graphics_set_callback", "_repopulate_perminvent", "_malloc"]'
EMCC_LFLAGS += -s EXPORTED_RUNTIME_METHODS='["cwrap", "ccall", "addFunction", \
			"removeFunction", "UTF8ToString", "stringToUTF8", "getValue", \
			"setValue", "ENV", "FS", "IDBFS"]'
EMCC_LFLAGS += -s ERROR_ON_UNDEFINED_SYMBOLS=0
EMCC_LFLAGS += -s EXPORT_ES6=1
EMCC_LFLAGS += -lidbfs.js
# XXX: the "@/" at the end of "--embed-file" tells emscripten to embed the files
# in the root directory, otherwise they will end up in the $(WASM_DATA_DIR) path
EMCC_LFLAGS += --embed-file $(WASM_DATA_DIR)@/
# For a list of EMCC settings:
# https://github.com/emscripten-core/emscripten/blob/master/src/settings.js
#
# WASM C flags
EMCC_CFLAGS =
EMCC_CFLAGS += -Wall
EMCC_CFLAGS += -Werror
EMCC_CFLAGS += -DNO_SIGNAL
#EMCC_CFLAGS += -s DISABLE_EXCEPTION_CATCHING=0
EMCC_DEBUG_LFLAGS += -s ASSERTIONS=1
EMCC_DEBUG_LFLAGS += -s ASSERTIONS=2
EMCC_DEBUG_LFLAGS += -s STACK_OVERFLOW_CHECK=2
EMCC_DEBUG_LFLAGS += -s SAFE_HEAP=1
EMCC_DEBUG_LFLAGS += -s LLD_REPORT_UNDEFINED=1
EMCC_DEBUG_LFLAGS += -s EXCEPTION_DEBUG=1
#EMCC_DEBUG_CFLAGS += -fsanitize=undefined -fsanitize=address -fsanitize=leak
EMCC_DEBUG_LFLAGS += -s NO_EXIT_RUNTIME
# XXX: if --profiling isn't included then any error dumps 10MB of WASM to the screen rather than a useful message
EMCC_DEBUG_LFLAGS += --profiling
EMCC_PROD_CFLAGS += -O3
ifdef WASM_DEBUG
EMCC_CFLAGS += $(EMCC_DEBUG_CFLAGS)
EMCC_LFLAGS += $(EMCC_DEBUG_LFLAGS)
else
EMCC_CFLAGS += $(EMCC_PROD_CFLAGS)
EMCC_LFLAGS += $(EMCC_PROD_LFLAGS)
endif
#
# Override the build tools and some obj files to
# reflect emscripten
override TARGET_CC = emcc
override TARGET_CXX = emcc
override TARGET_AR = emar
WASM_CFLAGS = -Wall -Wextra -Wno-missing-field-initializers
WASM_CFLAGS += -Wimplicit -Wreturn-type -Wunused -Wformat -Wswitch
WASM_CFLAGS += -Wshadow
# WASM_CFLAGS += -Wwrite-strings
# WASM_CFLAGS += -Werror
# Nethack C flags
WASM_CFLAGS += $(WINCFLAGS)   #WINCFLAGS set from multiw-2.500
WASM_CFLAGS += -DSYSCF -DSYSCF_FILE=\"/sysconf\" -DSECURE -DNHUUID
#WASM_CFLAGS += -g -I../include -DNOTPARMDECL
WASM_CFLAGS += -I../include -DNOTPARMDECL
# NetHack sources control
WASM_CFLAGS += -DDLB
WASM_CFLAGS += -DHACKDIR=\"$(HACKDIR)\"
WASM_CFLAGS += -DDEFAULT_WINDOW_SYS=\"shim\" \
#override TARGET_CFLAGS += -DGREPPATH=\"/usr/bin/grep\"
WASM_CFLAGS += -DNOMAIL
WASM_CFLAGS += $(LUAINCL)
WASM_CFLAGS += -DNOTTYGRAPHICS -DSHIM_GRAPHICS -DLIBNH
WASM_CFLAGS += -DCROSSCOMPILE
WASM_TARGET_CFLAGS = -DCROSSCOMPILE_TARGET -DCROSS_TO_WASM
# For src Makefile
override CFLAGS = $(WASM_CFLAGS)
override TARGET_CFLAGS = $(EMCC_CFLAGS) $(WASM_CFLAGS) $(WASM_TARGET_CFLAGS)
#
LUA_TARGET_CFLAGS = $(TARGET_CFLAGS)
override TARGET_CXXFLAGS = $(TARGET_CFLAGS)
override TARGET_LINK = $(TARGET_CC)
override TARGET_LFLAGS= $(EMCC_LFLAGS)
override SYSSRC = ../sys/libnh/libnhmain.c \
		../sys/share/ioctl.c ../sys/share/unixtty.c \
		../sys/unix/unixunix.c ../sys/unix/unixres.c \
		../win/shim/winshim.c
override SYSOBJ= $(TARGETPFX)libnhmain.o \
		$(TARGETPFX)ioctl.o $(TARGETPFX)unixtty.o \
		$(TARGETPFX)unixunix.o $(TARGETPFX)unixres.o \
		$(TARGETPFX)winshim.o
override WINLIB = emranlib
override LUALIB=
override TOPLUALIB=
override REGEXOBJ = $(TARGETPFX)posixregex.o
override WINOBJ=
# don't bother Making regular NetHack executable
override GAME=
# the real VARDAT hasn't been defined yet for use in ALLDEP override
WASM_DAT = bogusmon data engrave epitaph oracles quest.lua rumors
WASMDEP = include/nhlua.h $(WASM_DAT) spec_levs check-dlb
override ALLDEP = $(WASMDEP) wasm
override PREGAME += mkdir -p $(TARGETDIR)/wasm-data ;
override CLEANMORE += rm -rf $(TARGETDIR) ;
RANLIB=$(EMRANLIB)
#VARDATND += nhtiles.bmp
# Rule for file in sys/unix
$(TARGETPFX)%.o : ../sys/unix/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
# Rule for file in sys/libnh
$(TARGETPFX)%.o : ../sys/libnh/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
# Rule for files in win/shim
$(TARGETPFX)%.o : ../win/shim/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
endif  # CROSS_TO_WASM


ifdef CROSS_TO_MIPS
#================================================================
# MIPS cross-compile recipe
#================================================================
# Uses an MIPS linux cross-compiler on linux or macos.
#
# 1. You can obtain the cross-compiler for ubuntu via:
#     sudo apt -y install gcc-mipsel-linux-gnu g++-mipsel-linux-gnu
#    For macOS, perhaps check for a mips cross-compiler on
#    home-brew or macports.
#
# 2. Then
#     make CROSS_TO_MIPS=1 WANT_WIN_TTY=1 WANT_WIN_CURSES=1 all
#
#=================================================================

CFLAGS += -DCROSSCOMPILE

#
# Override the build tools and some obj files to
# reflect the mips cross-compiler.
#
override TARGET_CC = mipsel-linux-gnu-gcc
override TARGET_CXX = CXX=mipsel-linux-gnu-g++
override TARGET_AR = mipsel-linux-gnu-ar
MIPS_TARGET = $(TARGETPFX)nethack
#override TARGET_LINK = mipsel-linux-gnu-ld
MIPS_TARGET_CFLAGS = -c -O -I../include -I../sys/unix -I../win/share \
	$(LUAINCL) -DDLB \
	-DCROSSCOMPILE -DCROSSCOMPILE_TARGET \
	-DCROSS_TO_MIPS \
	-DCURSES_GENL_PUTMIXED \
        -Wall -Wextra -Wno-missing-field-initializers -Wreturn-type -Wunused \
        -Wformat -Wswitch -Wshadow -Wwrite-strings \
	-Wimplicit -Wimplicit-function-declaration -Wimplicit-int \
	-Wmissing-parameter-type -Wold-style-definition -Wstrict-prototypes \
	-Wno-unused-result
MIPS_TARGET_CXXFLAGS = -c -O -I../include -I../sys/unix -I../win/share \
        $(LUAINCL) -DDLB  \
        -DUSE_TILES -DCROSSCOMPILE -DCROSSCOMPILE_TARGET -DCROSS_TO_MIPS \
        -Wall -Wextra -Wno-missing-field-initializers -Wreturn-type -Wunused \
        -Wformat -Wswitch -Wshadow -Wwrite-strings -Wno-maybe-uninitialized
override TARGET_CFLAGS = $(MIPS_TARGET_CFLAGS) -Wmissing-declarations \
			-Wmissing-prototypes -pedantic -Wmissing-declarations \
			-Wformat-nonliteral
override TARGET_CXXFLAGS = $(MIPS_TARGET_CXXFLAGS)
ifdef CPLUSPLUS_NEEDED
override TARGET_LINK = mipsel-linux-gnu-gcc
else
override TARGET_LINK = mipsel-linux-gnu-g++
endif
override TARGET_LFLAGS=
override SYSOBJ = $(TARGETPFX)ioctl.o $(TARGETPFX)unixmain.o $(TARGETPFX)unixtty.o \
	$(TARGETPFX)unixunix.o $(TARGETPFX)unixres.o
override WINLIB = $(NCURSESLIB)
override LUALIB=
override LUALIBS=
override TOPLUALIB=
override GAMEBIN=$(MIPS_TARGET)
override RECOVERBIN = $(TARGETPFX)recover
override PACKAGE = mipspkg
override PREGAME += mkdir -p $(TARGETDIR) ;
override CLEANMORE += rm -f -r $(TARGETDIR) ;
NCURSES_CONFIGURE_BUILD=i686-pc-linux-gnu
NCURSES_CONFIGURE_HOST=mipsel-linux-gnu
NCURSES_PLATFORM=MIPS
# Rule for file in sys/unix
#$(TARGETPFX)%.o : ../sys/unix/%.c
#	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
# Rule for file in sys/share
#$(TARGETPFX)%.o : ../sys/share/%.c
#	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
endif  # CROSS_TO_MIPS
#=================================================================

#=================================================================

ifdef CROSS_TO_AMIGA
#===============-=================================================
# AmigaOS m68k cross-compile recipe
#===============-=================================================
# Uses an Amiga M68K cross-compiler on linux or macOS.
#
# Cross-compiler: https://franke.ms/git/bebbo/amiga-gcc
# Install to /opt/amiga, then:
#     sys/unix/setup.sh sys/unix/hints/linux.500
#     make fetch-lua
#     make CROSS_TO_AMIGA=1 fetch-regex
#     make CROSS_TO_AMIGA=1 all
#     make CROSS_TO_AMIGA=1 package
#=================================================================

CFLAGS += -DCROSSCOMPILE

#
# Override the build tools and some obj files to
# reflect the amiga-gcc cross-compiler.
#
TOOLTOP = /opt/amiga/bin
TOOLARCH = -m68000
override REGEXOBJ = $(TARGETPFX)posixregex.o
AMIREGEXOBJ = $(TARGETPFX)regcomp.o $(TARGETPFX)regexec.o \
	$(TARGETPFX)regerror.o $(TARGETPFX)regfree.o
override TARGET_CC = $(TOOLTOP)/m68k-amigaos-gcc
override TARGET_CXX = $(TOOLTOP)/m68k-amigaos-c++
override TARGET_AR = $(TOOLTOP)/m68k-amigaos-ar
override TARGET_STUBEDIT=
override TARGET_CFLAGS = -c -O2 -noixemul $(TOOLARCH) \
	-include sys/types.h \
	-I../include \
	-I../sys/amiga -I../win/share \
	$(LUAINCL) -DAMIGA -DNOTTYGRAPHICS -DNO_TERMS -DNO_SIGNAL \
	-DTILES_IN_GLYPHMAP $(PDCURSESDEF) \
	-DCROSSCOMPILE -DCROSSCOMPILE_TARGET -DCROSS_TO_AMIGA \
	-DAMIGA_VERSION_STRING=\""VER: NetHack 5.0.0"\"
override TARGET_CXXFLAGS = $(TARGET_CFLAGS)
LUA_TARGET_CFLAGS = $(TARGET_CFLAGS)
ifeq "$(REGEXOBJ)" "$(TARGETPFX)cppregex.o"
override TARGET_LINK = $(TARGET_CXX)
else
override TARGET_LINK = $(TARGET_CC)
endif
override TARGET_LFLAGS= $(TOOLARCH) -noixemul -Wl,--allow-multiple-definition
override TARGET_LIBS += $(LIBLM)
VARDATND += nhtiles.bmp
override SYSSRC = ../sys/amiga/amidos.c ../sys/amiga/amigst.c \
		../sys/amiga/amimenu.c ../sys/amiga/amirip.c \
		../sys/amiga/amistack.c ../sys/amiga/amitty.c \
		../sys/amiga/amiwind.c ../sys/amiga/clipwin.c \
		../sys/amiga/colorwin.c \
		../sys/amiga/winami.c ../sys/amiga/winchar.c \
		../sys/amiga/winfuncs.c ../sys/amiga/winkey.c \
		../sys/amiga/winamenu.c ../sys/amiga/winreq.c \
		../sys/amiga/winstr.c ../sys/share/pcmain.c \
		../win/share/bmptiles.c ../win/share/giftiles.c \
		../win/share/tileset.c
override SYSOBJ = $(TARGETPFX)amidos.o $(TARGETPFX)amigst.o \
		$(TARGETPFX)amirip.o $(TARGETPFX)amistack.o \
		$(TARGETPFX)amitty.o $(TARGETPFX)amiwind.o \
		$(TARGETPFX)winami.o $(TARGETPFX)winchar.o \
		$(TARGETPFX)winfuncs.o $(TARGETPFX)winkey.o \
		$(TARGETPFX)winamenu.o $(TARGETPFX)winreq.o \
		$(TARGETPFX)winstr.o $(TARGETPFX)pcmain.o \
		$(TARGETPFX)bmptiles.o $(TARGETPFX)giftiles.o \
		$(TARGETPFX)tileset.o \
		$(AMIREGEXOBJ)
override WINLIB=
override LUALIB=
override LUALIBS=
override TOPLUALIB=
override DLLIB=
override WINOBJ=
override GAMEBIN = $(TARGETPFX)nethack
override PACKAGE = amigapkg
override PREGAME += mkdir -p $(TARGETDIR) ;
override CLEANMORE += rm -r $(TARGETDIR) ;
#
# Rule for files in sys/amiga
$(TARGETPFX)%.o : ../sys/amiga/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -o$@ $<
# Rule for BSD regex in sys/amiga/regex
$(TARGETPFX)%.o : ../sys/amiga/regex/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -I../sys/amiga/regex -o$@ $<
endif  # CROSS_TO_AMIGA
#=================================================================

ifdef WANT_WIN_CURSES
ifdef BUILD_PDCURSES
# Rules for PDCurses files
$(TARGETPFX)%.o : $(PDCTOP)/pdcurses/%.c
	$(TARGET_CC) $(PDCINCL) $(PDC_TARGET_CFLAGS) -c -o$@  $<
endif  # BUILD_PDCURSES
endif  # WANT_WIN_CURSES

ifdef CROSS_SHARED
# Rules for win/share files
$(TARGETPFX)%.o : ../win/share/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
# Rules for util files heading for target
$(TARGETPFX)%.o : ../util/%.c
	$(TARGET_CC) $(TARGET_CFLAGS) -c -o$@ $<
endif  # CROSS_SHARED

ifdef BUILD_TARGET_LUA
# Rule for LUA files
$(TARGETPFX)%.o : $(LUATOP)/src/%.c
	$(TARGET_CC) $(LUA_TARGET_CFLAGS) -c $(LUA_FLAGS) -o$@ $<
endif  # BUILD_TARGET_LUA
#
# End of cross-compiling -PRE section
#===============-=================================================

#

#
# NetHack 5.0  gbdates-pre.500 $NHDT-Date: 1599687610 2020/09/09 21:40:10 $  $NHDT-Branch: NetHack-5.0 $:$NHDT-Revision: 1.11 $

ifdef MAKEFILE_DOC
ifneq "$(GIT)" "0"
#Get the hash of the last update to Guidebook.mn
GB_LAST_COMMIT := $(shell git log -n 1 --pretty=format:%H -- Guidebook.mn)
ifneq "$(GB_LAST_COMMIT)" ""
GB_DATESTAMP := $(shell git show -s --format=%cd --date=format:'%B %d, %Y' $(GB_LAST_COMMIT))
endif   # GB_LAST_COMMIT
ifneq "$(GB_DATESTAMP)" ""
DOC_EXTRAS += Guidebook.dated.mn Guidebook.dated.tex
override GUIDEBOOK_MN = Guidebook.dated.mn
override GUIDEBOOK_TEX = Guidebook.dated.tex
DOC_SPOTLESS_EXTRAS += Guidebook.pdf
endif   # GB_DATESTAMP
endif   # GIT=0 explicitly
endif   # MAKEFILE_DOC
#
#

#
#------------------------------------------------------------------------------
# NetHack 5.0  multisnd2-pre.500 $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $
#

ifdef NEEDS_WAV
WAVDIR = ../sound/wav
SNDWAVS = se_squeak_A se_squeak_B se_squeak_B_flat se_squeak_C se_squeak_D \
	se_squeak_D_flat se_squeak_E se_squeak_E_flat se_squeak_F \
	se_squeak_F_sharp se_squeak_G se_squeak_G_sharp sound_Bell \
	sound_Bugle_A sound_Bugle_B sound_Bugle_C sound_Bugle_D \
	sound_Bugle_E sound_Bugle_F sound_Bugle_G \
	sound_Drum_Of_Earthquake sound_Fire_Horn sound_Frost_Horn \
	sound_Leather_Drum sound_Magic_Harp_A sound_Magic_Harp_B \
	sound_Magic_Harp_C sound_Magic_Harp_D sound_Magic_Harp_E \
	sound_Magic_Harp_F sound_Magic_Harp_G sound_Tooled_Horn_A \
	sound_Tooled_Horn_B sound_Tooled_Horn_C sound_Tooled_Horn_D \
	sound_Tooled_Horn_E sound_Tooled_Horn_F sound_Tooled_Horn_G \
	sound_Wooden_Flute_A sound_Wooden_Flute_B sound_Wooden_Flute_C \
	sound_Wooden_Flute_D sound_Wooden_Flute_E sound_Wooden_Flute_F \
	sound_Wooden_Flute_G sound_Wooden_Harp_A sound_Wooden_Harp_B \
	sound_Wooden_Harp_C sound_Wooden_Harp_D sound_Wooden_Harp_E \
	sound_Wooden_Harp_F sound_Wooden_Harp_G sound_Magic_Flute_A \
	sound_Magic_Flute_B sound_Magic_Flute_C sound_Magic_Flute_D \
	sound_Magic_Flute_E sound_Magic_Flute_F sound_Magic_Flute_G \
	sa2_xpleveldown sa2_xplevelup

WAVS = $(addprefix $(WAVDIR)/, $(addsuffix .wav, $(SNDWAVS)))
endif   # NEEDS_WAV

# end of multisnd2-pre.500
#------------------------------------------------------------------------------

#

#
#------------------------------------------------------------------------------
# NetHack 5.0  response.500 $NHDT-Date: 1668359835 2022/11/13 17:17:15 $  $NHDT-Branch: NetHack-5.0 $

ifeq "$(RESP)" "1"
USE_RESPONSEFILE=1
endif
ifeq "$(resp)" "1"
USE_RESPONSEFILE=1
endif
ifeq "$(RESPONSE)" "1"
USE_RESPONSEFILE=1
endif
ifeq "$(response)" "1"
USE_RESPONSEFILE=1
endif

ifeq "$(USE_RESPONSEFILE)" "1"
RESPONSEFILES=create_responsefiles
CXXFLAGS = $(CCXXFLAGS) -I. -I$(QTDIR)/include $(QTCXXFLAGS)
CC_COMPILER_SWITCHES := $(subst \,\\,$(CFLAGS))
CC_COMPILER_SWITCHES := $(subst ",\",$(CC_COMPILER_SWITCHES))
CXX_COMPILER_SWITCHES := $(subst \,\\,$(CXXFLAGS))
CXX_COMPILER_SWITCHES := $(subst ",\",$(CXX_COMPILER_SWITCHES))
CC_RESPONSEFILE=../src/nethack_cc.rsp
CXX_RESPONSEFILE=../src/nethack_cxx.rsp
CFLAGS=@$(CC_RESPONSEFILE)
CXXFLAGS=@$(CXX_RESPONSEFILE)
CLEAN_CC_RESPONSEFILE=rm -f $(CC_RESPONSEFILE);
CLEAN_CXX_RESPONSEFILE=rm -f $(CXX_RESPONSEFILE);
endif

#end of response.500
#------------------------------------------------------------------------------
#

### End sys/unix/hints/linux.500 PRE

###
### Start Makefile.doc
###
#      NetHack Documentation Makefile.
# NetHack 5.0  Makefile.doc    $NHDT-Date: 1700788000 2023/11/24 01:06:40 $  $NHDT-Branch: NetHack-5.0 $:$NHDT-Revision: 1.28 $
# Copyright (c) 2015 by Kenneth Lorber, Kensington, Maryland
# NetHack may be freely redistributed.  See license for details.

# Root of source tree:
NHSROOT=..

# for Atari
# SHELL=E:/GEMINI2/MUPFEL.TTP

MAKEDEFS = ../util/makedefs

#STRIPBS ?= ../util/stripbs

# Which version do we want to build?  (XXX These are not used anywhere.)
GUIDEBOOK = Guidebook		# regular ASCII file
#GUIDEBOOK = Guidebook.ps	# PostScript file
#GUIDEBOOK = Guidebook.dvi	# TeX device-independent file

# Some versions of col need -x to keep them from converting spaces to tabs;
# some versions of col don't do the conversion by default and don't
# recognize the option.  Sigh.
#
# col is unnecessary, but harmless, with groff.  See grotty(1).
COLCMD ?= col -bx
#COLCMD ?= col -b
#COLCMD ?= $(STRIPBS)

# The command to use to generate a PostScript file
# PSCMD = ditroff | psdit
PSCMD = groff

# Use the "cat" GUIDECMD if nroff and/or tbl and/or col are not installed
# Not appropriate for creating Guidebook.txt.
# GUIDECMD = cat Guidebook.txt
#
# Single page. Might need adjustment to .pl value
# GUIDECMD= $(GUIDE_PREFORMAT) | perl -pe 's/^(.mt)$$/.pl 4720v\n.in 0\n.po 8\n.ll 64m\n$$1/' | nroff -c -Tascii | $(COLCMD)
#
GUIDECMD = $(GUIDE_PREFORMAT) | nroff -c -Tascii $(NROFF_FLAGS) | $(COLCMD)
ONEPAGECMD = $(ONEPAGE_PREFORMAT) | nroff -c -Tascii $(NROFF_FLAGS) | $(COLCMD)

# Only generate output for the current configuration:
NHGREP = $(MAKEDEFS) --grep --input - --output -
NEEDMAKEDEFS = $(MAKEDEFS)
# Generate output for all configurations:
#NHGREP = $(MAKEDEFS) --grep --input - --output - --grep-define ALLDOCS
#NEEDMAKEDEFS = $(MAKEDEFS)
# Fallback:
#NHGREP = cat
#NEEDMAKEDEFS =

GUIDEBOOK_MN_SRC = Guidebook.mn
GUIDEBOOK_MN = $(GUIDEBOOK_MN_SRC)
GUIDEBOOK_TEX_SRC = Guidebook.tex
GUIDEBOOK_TEX = $(GUIDEBOOK_TEX_SRC)

GUIDE_PREFORMAT = cat $(GUIDEBOOK_MN) | $(NHGREP) | tbl tmac.n -
# for Guidebook.dat, unpaginated version of Guidebook.txt
ONEPAGE_PREFORMAT = cat Gbk-1pg-pfx.mn $(GUIDEBOOK_MN) Gbk-1pg-sfx.mn \
	| $(NHGREP) | tbl tmac.n -

# the basic guidebook
Guidebook	: $(GUIDEBOOK_MN) tmac.n tmac.nh $(NEEDMAKEDEFS) $(STRIPBS)
	-$(GUIDECMD) > Guidebook

# Fancier output for those with ditroff, psdit and a PostScript printer.
# Could be converted to Guidebook.pdf if tool(s) for that are available.
Guidebook.ps	: $(GUIDEBOOK_MN) tmac.n tmac.nh $(NEEDMAKEDEFS)
	$(GUIDE_PREFORMAT) | $(PSCMD) > Guidebook.ps

# Guidebook.tex is the same as Guidebook.mn but formatted with LaTeX.
# - The invocation command for LaTeX may vary in different installations.
# - To print Guidebook.dvi you need to use a suitable dvi-driver.
# - LaTeX needs to be run twice; second pass uses Guidebook.aux made by first.
Guidebook.dvi	: $(GUIDEBOOK_TEX)
	latex $(GUIDEBOOK_TEX)
	latex $(GUIDEBOOK_TEX)

# makedefs has more dependencies than these; this is mainly to cope with the
# case where it hasn't been built yet since it is usually needed for $(NHGREP)
# (note: 'make makedefs', not 'make $(MAKEDEFS)')
$(MAKEDEFS)	: ../util/makedefs.c ../include/config.h ../src/mdlib.c \
		  ../util/mdgrep.h
	( cd .. ; make makedefs )

../util/stripbs: ../util/stripbs.c
	( cd .. ; $(MAKE) stripbs )

GAME	= nethack
MANDIR	?= /usr/man/man6
MANEXT	= 6
NROFF ?= nroff

# manual installation for most BSD-style systems
GAMEMANCREATE = cat nethack.6 | $(NHGREP) >
RCVRMANCREATE = cat recover.6 | $(NHGREP) >
DLBMANCREATE = cat dlb.6 | $(NHGREP) >
MDMANCREATE = cat makedefs.6 | $(NHGREP) >
# manual installation for most SYSV-style systems
# GAMEMANCREATE = cat nethack.6 | $(NHGREP) | nroff -man - >
# RCVRMANCREATE = cat recover.6 | $(NHGREP) | nroff -man - >
# DLBMANCREATE = cat dlb.6 | $(NHGREP) | nroff -man - >
# MDMANCREATE = cat makedefs.6 | $(NHGREP) | nroff -man - >

manpages: $(PREMANPAGES)
	-$(GAMEMANCREATE) $(MANDIR)/$(GAME).$(MANEXT)
	-$(RCVRMANCREATE) $(MANDIR)/recover.$(MANEXT)
	-$(DLBMANCREATE) $(MANDIR)/dlb.$(MANEXT)
	-$(MDMANCREATE) $(MANDIR)/makedefs.$(MANEXT)

# manual creation for distribution
DISTRIB = Guidebook.txt nethack.txt recover.txt \
	dlb.txt makedefs.txt

distrib: $(DISTRIB)
	@echo "Plain text documentation is up to date."

Guidebook.txt	: $(GUIDEBOOK_MN) tmac.n tmac.nh $(NEEDMAKEDEFS)
	$(GUIDECMD) > $@
Guidebook.dat	: Gbk-1pg-pfx.mn Gbk-1pg-sfx.mn $(GUIDEBOOK_MN) tmac.n tmac.nh \
		  $(NEEDMAKEDEFS)
	$(ONEPAGECMD) > $@

# plain text output
#
# MAN2TXTPRE and MAN2TXTPOST are included so that a hints file can alter
# both of them as required
MAN2TXTPOST ?= | col -b
MAN2TXT = $(NHGREP) | $(NROFF) -man $(MAN2TXTPRE) $(MAN2TXTPOST)
# If you aren't using a hints file, you can uncomment the following
# line if you have groff 1.23 or greater.
#MAN2TXT = $(NHGREP) | groff -man -Tascii -P -cbou
nethack.txt	: nethack.6
	cat nethack.6 | $(MAN2TXT) > nethack.txt
recover.txt	: recover.6
	cat recover.6 | $(MAN2TXT) > recover.txt
dlb.txt		: dlb.6
	cat dlb.6 | $(MAN2TXT) > dlb.txt
makedefs.txt		: makedefs.6
	cat makedefs.6 | $(MAN2TXT) > makedefs.txt

clean:
	-rm -f Guidebook.aux Guidebook.log $(DOC_EXTRAS)

spotless: clean
	-rm -f Guidebook Guidebook.dat Guidebook.ps \
		Guidebook.dvi $(DOC_SPOTLESS_EXTRAS)

maintainer-clean: spotless
	-rm -f $(DISTRIB)
#	-rm -f Makefile
### End Makefile.doc

###
### Start sys/unix/hints/linux.500 POST
###
# (new segment at source line 496 )

#
# NetHack 5.0  gbdates-post.500 $NHDT-Date: 1599687610 2020/09/09 21:40:10 $  $NHDT-Branch: NetHack-5.0 $:$NHDT-Revision: 1.11 $
ifdef MAKEFILE_DOC
ifdef GB_DATESTAMP
Guidebook.dated.mn: $(GUIDEBOOK_MN_SRC)
	@awk 'f{$$0=".ds f2 $(GB_DATESTAMP)";f=0}/NH_DATESUB/{f=1} 1' < $(GUIDEBOOK_MN_SRC) > $@
Guidebook.dated.tex: $(GUIDEBOOK_TEX_SRC)
	@awk 'f{$$0="\\date{$(GB_DATESTAMP)}";f=0}/NH_DATESUB/{f=1} 1' < $(GUIDEBOOK_TEX_SRC) > $@
endif   # GB_DATESTAMP

Guidebook.pdf: Guidebook.dated.tex
	pdflatex -interaction=nonstopmode -jobname=Guidebook Guidebook.dated.tex
	pdflatex -interaction=nonstopmode -jobname=Guidebook Guidebook.dated.tex
endif   # MAKEFILE_DOC
#
#
#
#------------------------------------------------------------------------------
# NetHack 5.0  multisnd-post.500 $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $

ifeq "$(HAVE_SNDLIB)" "1"
# note:	one possible value for $(SNDLIBSRC) is 'macsound.m' which is written
#	in Objective-C; this relies on $(CC) recognizing the filename suffix
#	and knowing how to handle that; clang on MacOS does, both when
#	masquerading as gcc and when invoked as itself
$(TARGETPFX)$(SNDLIBOBJ): $(SNDLIBSRC) $(HACK_H)
	$(CC) $(CFLAGS) -c -o $@ $(SNDLIBSRC)

ifeq "$(NEEDS_WAV)" "1"

$(WAVDIR)/%.wav: ../util/uudecode $(WAVDIR)/%.uu
	$^
	mv $(notdir $@) $@

../util/uudecode: ../sys/share/uudecode.c
	( cd ../util ; $(MAKE) uudecode )

endif  # NEEDS_WAV
endif  # HAVE_SNDLIB

# end of multisnd-post.500
#------------------------------------------------------------------------------

#

ifdef MAKEFILE_TOP
ifeq ($(MAKELEVEL),0)
.PHONY: checkmakefiles
checkmakefiles:
	@$(MAKE) -f sys/unix/Makefile.check \
		HINTSFILE="$(HINTSFILE)" HINTSINCLFILES="$(HINTSINCLFILES)"
endif
endif

ifdef WANT_LIBNH
$(TARGETPFX)libnh.a: $(HOBJ) $(LIBNHSYSOBJ) ../lib/lua/liblua-$(LUA_VERSION).a
	$(AR) rcs $@ $(HOBJ) $(LIBNHSYSOBJ) ../lib/lua/liblua-$(LUA_VERSION).a
	@echo "$@ built."
$(TARGETPFX)libnhmain.o : ../sys/libnh/libnhmain.c $(HACK_H)
	$(CC) $(CFLAGS) -c -o$@ $<
#dependency tool added this to Makefile.src
#$(TARGETPFX)winshim.o : ../win/shim/winshim.c $(HACK_H)
#	$(CC) $(CFLAGS) -c -o$@ $<
endif  # WANT_LIBNH

#
#===============-=================================================
# NetHack 5.0  include/cross-post $NHDT-Date: 1597332785 2020/08/13 15:33:05 $  $NHDT-Branch: NetHack-5.0 $
#
# Cross-compiling -POST section

ifdef CROSS_TO_MSDOS
#
$(TARGETPFX)msdos.o : ../sys/msdos/msdos.c $(HACK_H)
$(TARGETPFX)font.o : ../sys/msdos/font.c ../sys/msdos/font.h $(HACK_H)
$(TARGETPFX)pckeys.o : ../sys/msdos/pckeys.c $(HACK_H)
$(TARGETPFX)pctiles.o : ../sys/msdos/pctiles.c ../sys/msdos/portio.h $(HACK_H)
$(TARGETPFX)video.o : ../sys/msdos/video.c ../sys/msdos/portio.h $(HACK_H)
$(TARGETPFX)vidtxt.o : ../sys/msdos/vidtxt.c ../sys/msdos/portio.h \
		../win/share/tile.h ../include/tileset.h $(HACK_H)
$(TARGETPFX)vidvga.o : ../sys/msdos/vidvga.c ../sys/msdos/portio.h \
		../win/share/tile.h ../include/tileset.h $(HACK_H)
$(TARGETPFX)vidvesa.o : ../sys/msdos/vidvesa.c ../sys/msdos/portio.h \
		../win/share/tile.h ../include/tileset.h ../sys/msdos/font.h $(HACK_H)
$(TARGETPFX)vidstub.o : ../sys/msdos/vidvesa.c ../sys/msdos/portio.h \
		$(HACK_H)
$(TARGETPFX)tile.o : tile.c
$(TARGETPFX)exceptn.o : ../lib/djgpp/djgpp-patch/src/libc/go32/exceptn.S
	$(TARGET_CC) -c -o $@ ../lib/djgpp/djgpp-patch/src/libc/go32/exceptn.S
	$(TARGET_AR) ru ../lib/djgpp/i586-pc-msdosdjgpp/lib/libc.a $(TARGETPFX)exceptn.o
$(GAMEBIN) : $(HOBJ) $(TARGETPFX)date.o $(TARGET_HACKLIB) $(LUACROSSLIB)
	$(TARGET_LINK) $(TARGET_LFLAGS) -o $@ \
	$(HOBJ) $(TARGET_HACKLIB) $(WINLIB) $(TARGET_LIBS)
$(DOSFONT)/ter-u16b.psf: $(FONTTOP)/ter-u16b.bdf $(DOSFONT)/nh-u16b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u16b.bdf $(DOSFONT)/nh-u16b.bdf $@
$(DOSFONT)/ter-u16v.psf: $(FONTTOP)/ter-u16v.bdf $(DOSFONT)/nh-u16v.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u16v.bdf $(DOSFONT)/nh-u16v.bdf $@
$(DOSFONT)/ter-u18b.psf: $(FONTTOP)/ter-u18b.bdf $(DOSFONT)/nh-u18b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u18b.bdf $(DOSFONT)/nh-u18b.bdf $@
$(DOSFONT)/ter-u20b.psf: $(FONTTOP)/ter-u20b.bdf $(DOSFONT)/nh-u20b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u20b.bdf $(DOSFONT)/nh-u20b.bdf $@
$(DOSFONT)/ter-u22b.psf: $(FONTTOP)/ter-u22b.bdf $(DOSFONT)/nh-u22b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u22b.bdf $(DOSFONT)/nh-u22b.bdf $@
$(DOSFONT)/ter-u24b.psf: $(FONTTOP)/ter-u24b.bdf $(DOSFONT)/nh-u24b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u24b.bdf $(DOSFONT)/nh-u24b.bdf $@
$(DOSFONT)/ter-u28b.psf: $(FONTTOP)/ter-u28b.bdf $(DOSFONT)/nh-u28b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u28b.bdf $(DOSFONT)/nh-u28b.bdf $@
$(DOSFONT)/ter-u32b.psf: $(FONTTOP)/ter-u32b.bdf $(DOSFONT)/nh-u32b.bdf $(DOSFONT)/makefont.lua $(LUABIN)
	$(LUABIN) $(DOSFONT)/makefont.lua $(FONTTOP)/ter-u32b.bdf $(DOSFONT)/nh-u32b.bdf $@
#
.PHONY: dodata dospkg dosfonts
dosfonts: $(FONTTARGETS)
dospkg: dodata dosfonts $(GAMEBIN) $(TARGETPFX)recover.exe ../dat/nhtiles.bmp
	$(TARGET_STUBEDIT) $(GAMEBIN) minstack=2048K
	mkdir -p $(TARGETPFX)pkg
	cp $(GAMEBIN) $(TARGETPFX)pkg/NETHACK.EXE
	cp $(TARGETPFX)recover.exe $(TARGETPFX)pkg/RECOVER.EXE
	-cp $(SFCTOOLBIN) $(TARGETPFX)pkg/SFCTOOL.EXE
	cp ../dat/nhdat $(TARGETPFX)pkg/NHDAT
	cp ../dat/license $(TARGETPFX)pkg/LICENSE
	cp ../dat/nhtiles.bmp $(TARGETPFX)pkg/NHTILES.BMP
	cp ../dat/symbols $(TARGETPFX)pkg/SYMBOLS
	cp ../sys/share/NetHack.cnf $(TARGETPFX)pkg/NETHACK.CNF
	cp ../sys/msdos/sysconf $(TARGETPFX)pkg/SYSCONF
	cp ../doc/nethack.txt $(TARGETPFX)pkg/NETHACK.TXT
	-cp $(DOSFONT)/ter-u16b.psf $(TARGETPFX)pkg/TER-U16B.PSF
	-cp $(DOSFONT)/ter-u16v.psf $(TARGETPFX)pkg/TER-U16V.PSF
	-cp $(DOSFONT)/ter-u18b.psf $(TARGETPFX)pkg/TER-U18B.PSF
	-cp $(DOSFONT)/ter-u20b.psf $(TARGETPFX)pkg/TER-U20B.PSF
	-cp $(DOSFONT)/ter-u22b.psf $(TARGETPFX)pkg/TER-U22B.PSF
	-cp $(DOSFONT)/ter-u24b.psf $(TARGETPFX)pkg/TER-U24B.PSF
	-cp $(DOSFONT)/ter-u28b.psf $(TARGETPFX)pkg/TER-U28B.PSF
	-cp $(DOSFONT)/ter-u32b.psf $(TARGETPFX)pkg/TER-U32B.PSF
	cp  ../lib/djgpp/cwsdpmi/bin/CWSDPMI.EXE $(TARGETPFX)pkg/CWSDPMI.EXE
	( if [ -f ../lib/djgpp/target/bin/symify.exe ]; then \
	      cp  ../lib/djgpp/target/bin/symify.exe $(TARGETPFX)pkg/SYMIFY.EXE; \
          else \
              pwd; echo "../lib/djgpp/target/bin/symify.exe not found"; \
	  fi; )
ifeq "$(WANT_DEBUG)" "1"
	( if [ -f $(GDBEXE) ]; \
	  then \
		cp  $(GDBEXE) $(TARGETPFX)pkg/GDB.EXE; \
		echo "gdb -ex '$(GDBCMDLINE)' NETHACK.EXE"> $(TARGETPFX)pkg/$(GDBBAT); \
          else \
		pwd; echo "$(GDBEXE) not found and WANT_DEBUG=1 specified"; \
	  fi; )
else
	-( if [ -f $(TARGETPFX)pkg/GDB.EXE ]; \
	  then \
		rm $(TARGETPFX)pkg/GDB.EXE; \
	  fi; )
	-( if [ -f $(TARGETPFX)pkg/NHGDB.BAT ]; \
	  then \
		rm $(TARGETPFX)pkg/NHGDB.BAT; \
	  fi; )
endif
	-touch $(TARGETPFX)pkg/RECORD
	cd $(TARGETPFX)pkg ; zip -9 ../NH$(NHV)DOS.ZIP * ; cd ../../..
	@echo msdos package zip file $(TARGETPFX)NH$(NHV)DOS.ZIP

$(LUABIN):
	( cd .. && make luabin && cd src)
dodata:
	( cd .. && make dlb && cd src)
ifdef dosbox
# make CROSS_TO_MSDOS=1 dosbox=~/dosbox deploy-to-dosbox
ifdef MAKEFILE_TOP
deploy-to-dosbox: 
	( cd src; make $(DEPLOY); cd .. )
endif
.PHONY: deploytodosbox

deploytodosbox: $(TARGETPFX)NH$(NHV)DOS.ZIP $(dosboxnhfolder) $(dosboxnhsrc) \
			$(dosboxnhsrc)/src $(dosboxnhsrc)/include \
			$(dosboxnhsrc)/sys/msdos $(dosboxnhsrc)/sys/share \
			$(dosboxnhsrc)/win/share $(dosboxnhsrc)/win/curses \
			$(dosboxnhsrc)/win/tty $(dosboxnhsrc)/util \
			$(dosboxnhfolder)/NETHACK.EXE
	@echo DOS NetHack deployed to dosbox at $(dosboxnhfolder)
$(TARGETPFX)NH$(NHV)DOS.ZIP: dospkg
$(dosboxnhfolder):
	mkdir -p $@
$(dosboxnhsrc): $(dosboxnhfolder)
	mkdir -p $@
$(dosboxnhsrc)/src: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)src/*.c $(dosboxnhsrc)/src
$(dosboxnhsrc)/include: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)include/*.h $(dosboxnhsrc)/include
$(dosboxnhsrc)/sys/msdos: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)sys/msdos/*.c $(dosboxnhsrc)/sys/msdos
	cp --preserve=timestamps $(FLDR)sys/msdos/*.h $(dosboxnhsrc)/sys/msdos
$(dosboxnhsrc)/sys/share: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)sys/share/*.c $(dosboxnhsrc)/sys/share
$(dosboxnhsrc)/win/share: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)win/share/*.c $(dosboxnhsrc)/win/share
$(dosboxnhsrc)/win/curses: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)win/curses/*.c $(dosboxnhsrc)/win/curses
	cp --preserve=timestamps $(FLDR)win/curses/*.h $(dosboxnhsrc)/win/curses
$(dosboxnhsrc)/win/tty: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)win/tty/*.c $(dosboxnhsrc)/win/tty
$(dosboxnhsrc)/util: $(dosboxnhsrc)
	mkdir -p $@
	cp --preserve=timestamps $(FLDR)util/*.c $(dosboxnhsrc)/util
	cp --preserve=timestamps $(FLDR)util/*.h $(dosboxnhsrc)/util
$(dosboxnhfolder)/NETHACK.EXE: $(TARGETPFX)NH$(NHV)DOS.ZIP
	unzip -o $(TARGETPFX)NH$(NHV)DOS.ZIP -d $(dosboxnhfolder)
	find $(dosboxnhfolder) -type f -name "$(dosboxconfigfile)" \
		| xargs sed -i 's/#OPTIONS=video:autodetect/OPTIONS=video:autodetect/g'
endif  # dosbox
endif  # CROSS_TO_MSDOS

ifdef CROSS_TO_WASM
$(WASM_TARGET): pregame $(TARGET_HACKLIB) $(TARGETPFX)date.o $(HOSTOBJ) $(HOBJ) $(LUACROSSLIB) $(WASM_DATA_DIR)
	-rm $@
	$(TARGET_CC) $(TARGET_LFLAGS) $(TARGET_CFLAGS) -o $@ \
		$(HOBJ) $(TARGETPFX)date.o $(TARGET_HACKLIB) $(TARGET_LIBS)

$(WASM_DATA_DIR): $(WASM_DATA_DIR)/nhdat
	touch $(WASM_DATA_DIR)/perm
	touch $(WASM_DATA_DIR)/record
	touch $(WASM_DATA_DIR)/logfile
	touch $(WASM_DATA_DIR)/xlogfile
	touch $(WASM_DATA_DIR)/livelog
	cp ../sys/libnh/sysconf $(WASM_DATA_DIR)/sysconf

$(WASM_DATA_DIR)/nhdat:
	( cd ..; $(MAKE) INSTDIR='$(WASM_DATA_DIR)' $(WASMDEP) dofiles-dlb )

#
$(TARGETPFX)unixmain.o : ../sys/unix/unixmain.c $(HACK_H)
$(TARGETPFX)unixres.o : ../sys/unix/unixres.c $(HACK_H)
$(TARGETPFX)unixunix.o : ../sys/unix/unixunix.c $(HACK_H)
$(TARGETPFX)ioctl.o : ../sys/share/ioctl.c $(HACK_H)
$(TARGETPFX)unixtty.o : ../sys/share/unixtty.c $(HACK_H)
$(TARGETPFX)winshim.o : ../win/shim/winshim.c $(HACK_H)
$(TARGETPFX)libnhmain.o : ../sys/libnh/libnhmain.c $(HACK_H)
endif  # CROSS_TO_WASM

#
ifdef CROSS_TO_MIPS
$(MIPS_TARGET): pregame $(TARGETPFX)date.o $(HOSTOBJ) $(HOBJ) $(LUACROSSLIB) \
			$(TARGETPFX)ncurses/lib/libncurses.a \
			$(MIPS_DATA_DIR)
	-rm $@
	$(TARGET_LINK) $(TARGET_LFLAGS) -o $@ \
		$(HOBJ) $(TARGETPFX)date.o $(TARGETPFX)$(HACKLIB) \
		$(TARGETPFX)ncurses/lib/libncurses.a \
		$(TARGET_LIBS)

$(MIPS_DATA_DIR): $(MIPS_DATA_DIR)/nhdat
	touch $(MIPS_DATA_DIR)/perm
	touch $(MIPS_DATA_DIR)/record
	touch $(MIPS_DATA_DIR)/logfile
	touch $(MIPS_DATA_DIR)/xlogfile
	touch $(MIPS_DATA_DIR)/livelog
	cp ../sys/unix/sysconf $(MIPS_DATA_DIR)/sysconf

$(MIPS_DATA_DIR)/nhdat:
	( cd ..; $(MAKE) INSTDIR='$(MIPS_DATA_DIR)' $(MIPSDEP) dofiles-dlb )

#
$(TARGETPFX)unixmain.o : ../sys/unix/unixmain.c $(HACK_H)
$(TARGETPFX)unixres.o : ../sys/unix/unixres.c $(HACK_H)
$(TARGETPFX)unixunix.o : ../sys/unix/unixunix.c $(HACK_H)
$(TARGETPFX)ioctl.o : ../sys/share/ioctl.c $(HACK_H)
$(TARGETPFX)unixtty.o : ../sys/share/unixtty.c $(HACK_H)

$(LUABIN):
	( cd .. && make luabin && cd src)
dodata:
	( cd .. && make dlb && cd src)

mipsrecover: $(TARGETPFX)recover
.PHONY: mipspkg
mipspkg: dodata $(GAMEBIN) $(TARGETPFX)recover
	mkdir -p $(TARGETPFX)pkg
	cp $(GAMEBIN) $(TARGETPFX)pkg/nethack
	cp $(TARGETPFX)recover $(TARGETPFX)pkg/recover
	cp ../dat/nhdat $(TARGETPFX)pkg/nhdat
	cp ../dat/license $(TARGETPFX)pkg/license
	cp ../dat/symbols $(TARGETPFX)pkg/symbols
	cp ../sys/share/NetHack.cnf $(TARGETPFX)pkg/.nethackrc
	cp ../sys/msdos/sysconf $(TARGETPFX)pkg/sysconf
	cp ../doc/nethack.txt $(TARGETPFX)pkg/nethack.txt
	-touch $(TARGETPFX)pkg/record
	cd $(TARGETPFX)pkg ; zip -9 ../nh$(NHV)mips.zip * ; cd ../../..
	@echo MIPS package zip file $(TARGETPFX)nh$(NHV)mips.zip
endif  # CROSS_TO_MIPS


ifdef CROSS_TO_AMIGA
$(TARGETPFX)amidos.o : ../sys/amiga/amidos.c $(HACK_H)
$(TARGETPFX)amigst.o : ../sys/amiga/amigst.c $(HACK_H)
$(TARGETPFX)amirip.o : ../sys/amiga/amirip.c $(HACK_H)
$(TARGETPFX)amistack.o : ../sys/amiga/amistack.c $(HACK_H)
$(TARGETPFX)amitty.o : ../sys/amiga/amitty.c $(HACK_H)
$(TARGETPFX)amiwind.o : ../sys/amiga/amiwind.c \
		../sys/amiga/amimenu.c $(HACK_H)
$(TARGETPFX)winami.o : ../sys/amiga/winami.c $(HACK_H)
$(TARGETPFX)winchar.o : ../sys/amiga/winchar.c tile.c $(HACK_H)
$(TARGETPFX)winfuncs.o : ../sys/amiga/winfuncs.c $(HACK_H)
$(TARGETPFX)winkey.o : ../sys/amiga/winkey.c $(HACK_H)
$(TARGETPFX)winamenu.o : ../sys/amiga/winamenu.c $(HACK_H)
$(TARGETPFX)winreq.o : ../sys/amiga/winreq.c \
		../sys/amiga/colorwin.c \
		../sys/amiga/clipwin.c $(HACK_H)
$(TARGETPFX)winstr.o : ../sys/amiga/winstr.c $(HACK_H)
$(GAMEBIN) : $(HOBJ) $(LUACROSSLIB)
	$(TARGET_LINK) $(TARGET_LFLAGS) -o $(GAMEBIN) \
	$(HOBJ) $(WINLIB) $(TARGET_LIBS)
#
# Host-side IFF tile conversion tools (run on Linux, produce Amiga IFF files)
#
AMISRC = ../sys/amiga

$(TARGETPFX)xpm2iff_host: $(AMISRC)/xpm2iff_host.c
	$(CC) $(CFLAGS) -o $@ $<
$(TARGETPFX)tomb.iff: $(AMISRC)/grave16.xpm $(TARGETPFX)xpm2iff_host
	$(TARGETPFX)xpm2iff_host $(AMISRC)/grave16.xpm $@

$(TARGETPFX)bmp2iff_host: $(AMISRC)/bmp2iff_host.c
	$(CC) $(CFLAGS) -o $@ $<
$(TARGETPFX)tiles16.iff: ../dat/nhtiles.bmp $(TARGETPFX)bmp2iff_host
	$(TARGETPFX)bmp2iff_host -planes 4 ../dat/nhtiles.bmp $@
$(TARGETPFX)tiles32.iff: ../dat/nhtiles.bmp $(TARGETPFX)bmp2iff_host
	$(TARGETPFX)bmp2iff_host -planes 5 ../dat/nhtiles.bmp $@

AMITILES = $(TARGETPFX)tiles16.iff $(TARGETPFX)tiles32.iff $(TARGETPFX)tomb.iff

AMIREGEX_URL = https://github.com/garyhouston/regex.git
AMIREGEX_SRCDIR = $(AMISRC)/regex

.PHONY: fetch-regex amigapkg amitiles

fetch-regex:
	@DSTDIR=sys/amiga/regex; \
	if [ ! -d src ]; then DSTDIR=../$$DSTDIR; fi; \
	if [ -f $$DSTDIR/regcomp.c ]; then \
	  echo "BSD regex already present"; \
	else \
	  echo "Fetching BSD regex from $(AMIREGEX_URL)"; \
	  tmpdir=$$(mktemp -d) && \
	  git clone --depth 1 $(AMIREGEX_URL) $$tmpdir && \
	  cd $$tmpdir && \
	  sh ./mkh -p regcomp.c > regcomp.ih && \
	  sh ./mkh -p engine.c > engine.ih && \
	  sh ./mkh -p regexec.c > regexec.ih && \
	  sh ./mkh -p regerror.c > regerror.ih && \
	  sh ./mkh -i _REGEX_H_ regex2.h regcomp.c \
	    regexec.c regerror.c regfree.c > regex.h && \
	  cd - > /dev/null && \
	  mkdir -p $$DSTDIR && \
	  for f in regcomp.c regexec.c regerror.c regfree.c \
	    engine.c regex.h regex2.h cclass.h cname.h \
	    utils.h regcomp.ih engine.ih regexec.ih \
	    regerror.ih \
	    COPYRIGHT; do \
	    cp $$tmpdir/$$f $$DSTDIR/; \
	  done && \
	  rm -rf $$tmpdir && \
	  echo "BSD regex installed in $$DSTDIR"; \
	fi
amitiles: $(AMITILES)

UUDECODE = ../util/uudecode

../util/uudecode: ../sys/share/uudecode.c
	$(CC) $(CFLAGS) -o $@ $<

amigapkg: $(AMITILES) ../util/uudecode
	mkdir -p $(TARGETPFX)pkg/NetHack/tiles $(TARGETPFX)pkg/NetHack/hack
	cp $(GAMEBIN) $(TARGETPFX)pkg/NetHack/nethack
	cp ../dat/nhdat $(TARGETPFX)pkg/NetHack/nhdat
	cp ../dat/license $(TARGETPFX)pkg/NetHack/license
	cp ../dat/symbols $(TARGETPFX)pkg/NetHack/symbols
	cp $(TARGETPFX)tiles16.iff $(TARGETPFX)pkg/NetHack/tiles/tiles16.iff
	cp $(TARGETPFX)tiles32.iff $(TARGETPFX)pkg/NetHack/tiles/tiles32.iff
	cp $(TARGETPFX)tomb.iff $(TARGETPFX)pkg/NetHack/tomb.iff
	cp ../doc/nethack.txt $(TARGETPFX)pkg/NetHack/nethack.txt
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/amifont8.uu && mv 8 hack/8 )
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/amifont.uu )
	cp $(AMISRC)/nethack.cnf $(TARGETPFX)pkg/NetHack/nethack.cnf
	cp $(AMISRC)/README.amiga $(TARGETPFX)pkg/NetHack/README.amiga
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/NHinfo.uu )
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/cnf-info.uu )
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/license-info.uu )
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/txt-info.uu )
	( cd $(TARGETPFX)pkg/NetHack && ../../../../util/uudecode ../../../../sys/amiga/readme-info.uu )
	( cd $(TARGETPFX)pkg && ../../../util/uudecode ../../../sys/amiga/drawer-info.uu )
	touch $(TARGETPFX)pkg/NetHack/record
	( cd $(TARGETPFX)pkg && zip -9r ../NH$(NHV)AMI.ZIP NetHack NetHack.info )
	@echo amiga package zip file $(TARGETPFX)NH$(NHV)AMI.ZIP
endif  # CROSS_TO_AMIGA

ifdef CROSS_SHARED
# shared file dependencies
$(TARGETPFX)pcmain.o : ../sys/share/pcmain.c $(HACK_H)
$(TARGETPFX)pcsys.o : ../sys/share/pcsys.c $(HACK_H)
$(TARGETPFX)pctty.o : ../sys/share/pctty.c $(HACK_H)
$(TARGETPFX)pcunix.o : ../sys/share/pcunix.c $(HACK_H)
$(TARGETPFX)tileset.o : ../win/share/tileset.c
$(TARGETPFX)bmptiles.o : ../win/share/bmptiles.c
$(TARGETPFX)giftiles.o : ../win/share/giftiles.c
endif  # CROSS_SHARED
#
ifdef CROSS
$(TARGETPFX)hacklib.a: $(TARGETPFX)hacklib.o
	$(TARGET_AR) $(TARGET_ARFLAGS) $@ $(TARGETPFX)hacklib.o
ifdef MAKEFILE_UTL
$(TARGETPFX)recover.o: recover.c $(CONFIG_H)
	$(TARGET_CC) $(TARGET_CFLAGS) $(CSTD) -c recover.c -o $@
ifdef CROSS_TO_MSDOS
$(TARGETPFX)recover.exe : $(TARGETPFX)recover.o $(TARGETPFX)rversion.o $(TARGETPFX)hacklib.a
	$(TARGET_LINK) $(TARGET_LFLAGS) \
	$(TARGETPFX)recover.o $(TARGETPFX)rversion.o $(TARGETPFX)hacklib.a -o $@
else
$(TARGETPFX)recover : $(TARGETPFX)recover.o $(TARGETPFX)rversion.o $(TARGETPFX)hacklib.a
	$(TARGET_LINK) $(TARGET_LFLAGS) \
	$(TARGETPFX)recover.o $(TARGETPFX)rversion.o $(TARGETPFX)hacklib.a -o $@
endif
$(TARGETPFX)rversion.o: ../src/version.c $(HACK_H)
	$(TARGET_CC) $(TARGET_CFLAGS) -DMINIMAL_FOR_RECOVER  $(CSTD) -c ../src/version.c -o $@
endif  # MAKEFILE_UTL
endif  # CROSS

ifdef BUILD_TARGET_LUA
# Lua lib
$(LUACROSSLIB): $(LUALIBOBJS)
	if [ -f $@ ]; then rm $@; fi;
	$(TARGET_AR) rcS $@ $(LUAOBJFILES1)
	$(TARGET_AR) rcS $@ $(LUAOBJFILES2)
	$(TARGET_AR) rcS $@ $(LUAOBJFILES3)
	$(TARGET_AR) rcs $@ $(LUAOBJFILES4)

#	$(TARGET_AR) rcs $@ $(LUALIBOBJS)

# Lua src
$(TARGETPFX)lapi.o : $(LUATOP)/src/lapi.c
$(TARGETPFX)lauxlib.o : $(LUATOP)/src/lauxlib.c
$(TARGETPFX)lbaselib.o : $(LUATOP)/src/lbaselib.c
$(TARGETPFX)lbitlib.o : $(LUATOP)/src/lbitlib.c
$(TARGETPFX)lcode.o : $(LUATOP)/src/lcode.c
$(TARGETPFX)lcorolib.o : $(LUATOP)/src/lcorolib.c
$(TARGETPFX)lctype.o : $(LUATOP)/src/lctype.c
$(TARGETPFX)ldblib.o : $(LUATOP)/src/ldblib.c
$(TARGETPFX)ldebug.o : $(LUATOP)/src/ldebug.c
$(TARGETPFX)ldo.o : $(LUATOP)/src/ldo.c
$(TARGETPFX)ldump.o : $(LUATOP)/src/ldump.c
$(TARGETPFX)lfunc.o : $(LUATOP)/src/lfunc.c
$(TARGETPFX)lgc.o : $(LUATOP)/src/lgc.c
$(TARGETPFX)linit.o : $(LUATOP)/src/linit.c
$(TARGETPFX)liolib.o : $(LUATOP)/src/liolib.c
$(TARGETPFX)llex.o : $(LUATOP)/src/llex.c
$(TARGETPFX)lmathlib.o : $(LUATOP)/src/lmathlib.c
$(TARGETPFX)lmem.o : $(LUATOP)/src/lmem.c
$(TARGETPFX)loadlib.o : $(LUATOP)/src/loadlib.c
$(TARGETPFX)lobject.o : $(LUATOP)/src/lobject.c
$(TARGETPFX)lopcodes.o : $(LUATOP)/src/lopcodes.c
$(TARGETPFX)loslib.o : $(LUATOP)/src/loslib.c
$(TARGETPFX)lparser.o : $(LUATOP)/src/lparser.c
$(TARGETPFX)lstate.o : $(LUATOP)/src/lstate.c
$(TARGETPFX)lstring.o : $(LUATOP)/src/lstring.c
$(TARGETPFX)lstrlib.o : $(LUATOP)/src/lstrlib.c
$(TARGETPFX)ltable.o : $(LUATOP)/src/ltable.c
$(TARGETPFX)ltablib.o : $(LUATOP)/src/ltablib.c
$(TARGETPFX)ltm.o : $(LUATOP)/src/ltm.c
$(TARGETPFX)lundump.o : $(LUATOP)/src/lundump.c
$(TARGETPFX)lutf8lib.o : $(LUATOP)/src/lutf8lib.c
$(TARGETPFX)lvm.o : $(LUATOP)/src/lvm.c
$(TARGETPFX)lzio.o : $(LUATOP)/src/lzio.c
endif  # BUILD_TARGET_LUA

ifdef BUILD_TARGET_NCURSES
.PHONY: build-ncurses
ifdef MAKEFILE_SRC
../lib/ncurses.tar.gz:
	@echo "You will need to successfully execute 'make CROSS_TO_$(NCURSES_PLATFORM)=1 fetch-ncurses' first"
	@false
$(TARGETPFX)ncurses/lib/libncurses.a: ../lib/ncurses.tar.gz
	(cd $(TARGETDIR) ; mkdir -p ncurses ; cd ncurses ; tar -xf ../../../lib/ncurses.tar.gz --strip-components=1 ; \
	./configure --build $(NCURSES_CONFIGURE_BUILD) --host $(NCURSES_CONFIGURE_HOST) ; \
	make ; \
	cd ../../../src)
endif   #MAKEFILE_SRC
ifdef MAKEFILE_TOP
.PHONY: fetch-ncurses
fetch-ncurses:
	(cd lib ; curl https://invisible-island.net/datafiles/release/ncurses.tar.gz --output ncurses.tar.gz ; cd ..)
endif
endif   #BUILD_TARGET_NCURSES

ifdef BUILD_PDCURSES
ifdef WANT_WIN_CURSES
$(TARGETPFX)pdclib.a : $(PDCLIBOBJS) $(PDCOBJS)
	if [ -f $@ ]; then rm $@; fi;
	$(TARGET_AR) rcs $@ $(PDCLIBOBJS) $(PDCOBJS)
endif  #WANT_WIN_CURSES
# PDCurses src
$(TARGETPFX)addch.o : $(PDCTOP)/pdcurses/addch.c
$(TARGETPFX)addchstr.o : $(PDCTOP)/pdcurses/addchstr.c
$(TARGETPFX)addstr.o : $(PDCTOP)/pdcurses/addstr.c
$(TARGETPFX)attr.o : $(PDCTOP)/pdcurses/attr.c
$(TARGETPFX)beep.o : $(PDCTOP)/pdcurses/beep.c
$(TARGETPFX)bkgd.o : $(PDCTOP)/pdcurses/bkgd.c
$(TARGETPFX)border.o : $(PDCTOP)/pdcurses/border.c
$(TARGETPFX)clear.o : $(PDCTOP)/pdcurses/clear.c
$(TARGETPFX)color.o : $(PDCTOP)/pdcurses/color.c
$(TARGETPFX)delch.o : $(PDCTOP)/pdcurses/delch.c
$(TARGETPFX)deleteln.o : $(PDCTOP)/pdcurses/deleteln.c
$(TARGETPFX)getch.o : $(PDCTOP)/pdcurses/getch.c
$(TARGETPFX)getstr.o : $(PDCTOP)/pdcurses/getstr.c
$(TARGETPFX)getyx.o : $(PDCTOP)/pdcurses/getyx.c
$(TARGETPFX)inch.o : $(PDCTOP)/pdcurses/inch.c
$(TARGETPFX)inchstr.o : $(PDCTOP)/pdcurses/inchstr.c
$(TARGETPFX)initscr.o : $(PDCTOP)/pdcurses/initscr.c
$(TARGETPFX)inopts.o : $(PDCTOP)/pdcurses/inopts.c
$(TARGETPFX)insch.o : $(PDCTOP)/pdcurses/insch.c
$(TARGETPFX)insstr.o : $(PDCTOP)/pdcurses/insstr.c
$(TARGETPFX)instr.o : $(PDCTOP)/pdcurses/instr.c
$(TARGETPFX)kernel.o : $(PDCTOP)/pdcurses/kernel.c
$(TARGETPFX)keyname.o : $(PDCTOP)/pdcurses/keyname.c
$(TARGETPFX)mouse.o : $(PDCTOP)/pdcurses/mouse.c
$(TARGETPFX)move.o : $(PDCTOP)/pdcurses/move.c
$(TARGETPFX)outopts.o : $(PDCTOP)/pdcurses/outopts.c
$(TARGETPFX)overlay.o : $(PDCTOP)/pdcurses/overlay.c
$(TARGETPFX)pad.o : $(PDCTOP)/pdcurses/pad.c
$(TARGETPFX)panel.o : $(PDCTOP)/pdcurses/panel.c
$(TARGETPFX)printw.o : $(PDCTOP)/pdcurses/printw.c
$(TARGETPFX)refresh.o : $(PDCTOP)/pdcurses/refresh.c
$(TARGETPFX)scanw.o : $(PDCTOP)/pdcurses/scanw.c
$(TARGETPFX)scr_dump.o : $(PDCTOP)/pdcurses/scr_dump.c
$(TARGETPFX)scroll.o : $(PDCTOP)/pdcurses/scroll.c
$(TARGETPFX)slk.o : $(PDCTOP)/pdcurses/slk.c
$(TARGETPFX)termattr.o : $(PDCTOP)/pdcurses/termattr.c
$(TARGETPFX)touch.o : $(PDCTOP)/pdcurses/touch.c
$(TARGETPFX)util.o : $(PDCTOP)/pdcurses/util.c
$(TARGETPFX)window.o : $(PDCTOP)/pdcurses/window.c
$(TARGETPFX)debug.o : $(PDCTOP)/pdcurses/debug.c
$(TARGETPFX)pdcclip.o : $(PDCPORT)/pdcclip.c
$(TARGETPFX)pdcdisp.o : $(PDCPORT)/pdcdisp.c
$(TARGETPFX)pdcgetsc.o : $(PDCPORT)/pdcgetsc.c
$(TARGETPFX)pdckbd.o : $(PDCPORT)/pdckbd.c
$(TARGETPFX)pdcscrn.o : $(PDCPORT)/pdcscrn.c
$(TARGETPFX)pdcsetsc.o : $(PDCPORT)/pdcsetsc.c
$(TARGETPFX)pdcutil.o : $(PDCPORT)/pdcutil.c
	$(TARGET_CC) $(PDCINCL) $(PDC_TARGET_CFLAGS) \
		-Wno-sign-compare -o$@  $(PDCPORT)/pdcutil.c
# -Wno-sign-compare
endif  # BUILD_PDCURSES
#
# End of cross-compiling -POST section
#===============-=================================================
#

### End sys/unix/hints/linux.500 POST
