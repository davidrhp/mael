BIN_PATH=/Users/davidrhp/rust/mael/target/debug/unique

set -o allexport
source .env
set +o allexport

if [[ -z "${MAELSTROM_PATH}" ]]; then
 echo "MAELSTROM_PATH not set"
 exit -1
fi

cargo build && \
$MAELSTROM_PATH test -w unique-ids --bin $BIN_PATH --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition