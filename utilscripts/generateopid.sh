#!/usr/bin/bash

ZADDR=zregtestsapling1l7wz8acgc79sp6t722w9xe0e7nas22rm7pqnvknf24wj7vw65pvsc2m2a00kf72ayx2tsgy5stq
ZADDR2=zregtestsapling1lr48dd3luv708m55ja3rrn2aaxcawqzv0pn2epsrrq3junkwzaz7wuyj3q6p68rx94zms39kxnc
zcash-cli generate 20
OPID=`zcash-cli z_sendmany $ZADDR "[{\"address\":\"$ZADDR2\", \"amount\":5.0}]"`
zcash-cli z_getoperationstatus "[\"$OPID\"]"
#ZADDR2=`zcash-cli z_getnewaddress`
#zcash-cli generate 20
#zcash-cli z_shieldcoinbase * $ZADDR
#OPID=`zcash-cli z_sendmany $ZADDR "[{\"address\":\"$ZADDR2\",\"amount\":0.01}]"`
#echo $OPID
