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
time make && ./src/zcashd &
# 2.5 seconds appears to be close to the minimum necessary boot time
sleep 2.5
$ZCASHROOT/src/zcash-cli help getblockchaininfo
cd $QUIZFACEROOT
#$QUIZFACEROOT/utilscripts/run_quizface.sh
