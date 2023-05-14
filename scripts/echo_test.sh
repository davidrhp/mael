BIN_PATH=/Users/davidrhp/rust/mael/target/debug/echo

set -o allexport
source .env
set +o allexport

if [[ -z "${MAELSTROM_PATH}" ]]; then
 echo "MAELSTROM_PATH not set"
 exit -1
fi

cargo build && \
$MAELSTROM_PATH test -w echo --bin $BIN_PATH --node-count 1 --time-limit 10