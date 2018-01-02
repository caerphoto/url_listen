BM=/tmp/bookmark
touch $BM
printf "javascript:" > $BM
cat bookmarklet.js >> $BM
cat $BM | pbcopy
