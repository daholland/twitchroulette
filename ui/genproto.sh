# Path to this plugin
PROTOC_GEN_TS_PATH="./node_modules/.bin/protoc-gen-ts"

# Directory to write generated code to (.js and .d.ts files)
OUT_DIR="./src/service"

protoc \
  --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
  -I ../proto \
  --js_out=import_style=commonjs,binary:${OUT_DIR} \
  --ts_out=service=grpc-web:${OUT_DIR} \
  ../proto/twitchroulette/v1/twitchroulette.proto \