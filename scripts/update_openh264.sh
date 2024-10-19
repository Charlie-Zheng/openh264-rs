#
# Updates our copy of OpenH264 from an upstream folder.
#

PROJECT_ROOT="$( cd "$(dirname "$0")/.." ; pwd -P )" # this file

VANILLA_UPSTREAM="$PROJECT_ROOT/../_thirdparty/openh264" # <--- BEFORE RUNNING THIS SCRIPT YOU PROBABLY WANT TO UPDATE THIS PATH
OUR_UPSTREAM="$PROJECT_ROOT/openh264-sys2/upstream"
FIND=/usr/bin/find

mkdir -p "$OUR_UPSTREAM"

# COPY WANTED FILES
cp -r "$VANILLA_UPSTREAM/codec" "$OUR_UPSTREAM"
cp "$VANILLA_UPSTREAM/README.md" "$OUR_UPSTREAM"
cp "$VANILLA_UPSTREAM/LICENSE" "$OUR_UPSTREAM"

# DELETE UNWANTED FILES
rm -rf "$OUR_UPSTREAM/codec/build"
$FIND "$OUR_UPSTREAM" -name "*.d" -delete
$FIND "$OUR_UPSTREAM" -name "*.o" -delete
$FIND "$OUR_UPSTREAM" -name "*.build" -delete
$FIND "$OUR_UPSTREAM" -name "*.mk" -delete
$FIND "$OUR_UPSTREAM" -name "*.sh" -delete

# Update version info
pushd "$VANILLA_UPSTREAM"
rm -f "$OUR_UPSTREAM/VERSION"
git config --get remote.origin.url >> "$OUR_UPSTREAM/VERSION"
git rev-parse HEAD >> "$OUR_UPSTREAM/VERSION"
popd