#!/usr/bin/env sh

# Push
git add .
currentTime=`date +%H:%M/%x`
git commit -m "$currentTime"
git push -f

echo Update Successfully!

exit 0