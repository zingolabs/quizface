#! /usr/bin/sh

# Load Variable Values
source ${HOME}/.config/quizface.conf

# Update zcashd and start it
killall zcashd
set -e
cd $ZCASHROOT
time make
./src/zcashd -conf=$QUIZFACEROOT/utilscripts/pipeline_zcash.conf &
# 2.5 seconds appears to be close to the minimum necessary boot time
sleep 2.5
cd $QUIZFACEROOT
cat $QUIZFACEROOT/lists/passing.txt | PATH=$PATH:$ZCASHROOT/src xargs cargo run
QUIZFOUT=$QUIZFACEROOT/output/`ls -1rct $QUIZFACEROOT/output/ | tail -n 1`
cd $ZCASHRPCROOT && cargo test --workspace
cd $TYPEGENROOT && cargo run $QUIZFOUT
