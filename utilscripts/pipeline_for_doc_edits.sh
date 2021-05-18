#! /usr/bin/sh

# Load Variable Values
source ${HOME}/.config/quizface.conf

# Update zcashd and start it
killall zcashd
set -ex
cd $ZCASHROOT
time make -j$(nproc)
./src/zcashd -conf=$QUIZFACEROOT/utilscripts/regtest_zcash.conf &
# 2.5 seconds appears to be close to the minimum necessary boot time
sleep 2.5
cd $QUIZFACEROOT
cargo build -q
cargo doc -q 
cargo test -q 
rm -r output/metascrubbing_0.2.0
cat $QUIZFACEROOT/lists/passing.txt | PATH=$PATH:$ZCASHROOT/src xargs cargo run -q
QUIZFOUT=$QUIZFACEROOT/output/`ls -1rct $QUIZFACEROOT/output/ | tail -n 1`
cd $TYPEGENROOT 
cargo test -q --workspace 
cargo run -q $QUIZFOUT
cd $ZCASHRPCROOT 
cargo test --workspace
cd $ZCASHROOT 
./src/zcash-gtest --gtest_filter=rpc.CheckExperimentalDisabledHelpMsg
