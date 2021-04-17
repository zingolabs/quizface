#! /usr/bin/sh

# to use:
# cd to the root of quizface
# ZCASHROOT=WHERE_EVER QUIZFACEROOT=splasm TYPEGENROOT=woosh ./utilscripts/pipeline_for_doc_edits.sh
# 

set -x

# Update zcashd and start it
cd $ZCASHROOT
killall zcashd
set -e
time make
./src/zcashd -d &

# Run quizface
cd $QUIZFACEROOT
echo `pwd`
