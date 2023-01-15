#!/bin/sh
DIR="`dirname "$0"`"
HDT_JAVA_HOME=`ls -td $DIR/hdt-java-package-3* | head -1`
HDT_JAVA_CP="$HDT_JAVA_HOME/lib/*"

echo "$DIR:$CLASSPATH:$HDT_JAVA_CP"
