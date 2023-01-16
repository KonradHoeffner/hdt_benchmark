#!/bin/sh
DIR="`dirname "$0"`"
rm $DIR/../data/persondata*.hdt.index* 2>/dev/null
$DIR/run $@
