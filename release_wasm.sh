set -e

# makes a release zip of the game for distribution

rust_project_name=quads_jam_2024
itchio_user=nathanleiby
itchio_game_name=quads-jam-2024

rm web-release.zip || true
rm -rf web-release
mkdir -p web-release
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/$rust_project_name.wasm web-release
cp web/*js web-release
cp web/index.html web-release
mkdir -p web-release/assets
cp -r assets/* web-release/assets
rm -rf web-release/**/.DS_Store
rm -rf web-release/**/*/.DS_Store
zip -r web-release.zip web-release
echo "web release zipped into web-release.zip"
butler push web-release.zip $itchio_user/$itchio_game_name:html
