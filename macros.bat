@echo off

rem ----------
rem git macros
rem ----------
doskey gst=git status --show-stash
doskey gco=git checkout $*
doskey gbn=git branch --show-current
doskey ga=git add $*
doskey gcam=git commit -m $*

rem ------
rem others
rem ------
doskey t=tree /f $*
doskey subl="c:\Program Files\Sublime Text 3\subl.exe" $*