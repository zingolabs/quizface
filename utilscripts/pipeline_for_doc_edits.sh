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
cd $QUIZFACEROOT
cat $QUIZFACEROOT/lists/passing.txt | \
    PATH=$PATH:$ZCASHROOT/src xargs cargo run && \
    QUIZFOUT=$QUIZFACEROOT/output/`ls -1rct $QUIZFACEROOT/output/ | \
    tail -n 1` && \
    cd $ZCASHRPCROOT && cargo test --workspace && \
    cd $TYPEGENROOT && cargo run $QUIZFOUT
