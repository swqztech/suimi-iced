#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PACKAGING="$ROOT/packaging"
STAGE="$PACKAGING/stage"
DIST="$ROOT/dist"
INSTALLER_MODE="${INSTALLER:-both}"
VERSION="$(grep -E '^\s*version\s*=' "$ROOT/Cargo.toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"

(cd "$ROOT" && cargo build --release)
EXE_SRC="$ROOT/target/release/suimi-iced.exe"
[[ -f "$EXE_SRC" ]] || exit 1

PORTABLE_NAME="随密-iced-${VERSION}-win64-绿色版"
PORTABLE_DIR="$DIST/$PORTABLE_NAME"
rm -rf "$DIST"
mkdir -p "$PORTABLE_DIR"
cp -f "$EXE_SRC" "$PORTABLE_DIR/随密-iced.exe"
cp -f "$STAGE/启动.bat" "$STAGE/使用说明.txt" "$PORTABLE_DIR/"
(cd "$DIST" && tar -a -cf "${PORTABLE_NAME}.zip" "$(basename "$PORTABLE_DIR")")

INSTALLER_STAGE="$PACKAGING/installer-stage"
rm -rf "$INSTALLER_STAGE"
mkdir -p "$INSTALLER_STAGE"
cp -r "$PORTABLE_DIR"/* "$INSTALLER_STAGE/"

SETUP_EXE="$DIST/随密-iced-${VERSION}-win64-setup.exe"
SETUP_ASCII="$DIST/Suimi-iced-${VERSION}-win64-setup.exe"
BUILT=0

if [[ "$INSTALLER_MODE" == "nsis" || "$INSTALLER_MODE" == "both" ]]; then
  for p in "/c/Program Files (x86)/NSIS/makensis.exe" "/c/Program Files/NSIS/makensis.exe"; do
    if [[ -f "$p" ]]; then
      "$p" "//INPUTCHARSET" "UTF8" "//DVERSION=$VERSION" \
        "//DSTAGE_DIR=$(cygpath -w "$INSTALLER_STAGE")" \
        "//DOUTFILE=$(cygpath -w "$SETUP_ASCII")" \
        "$(cygpath -w "$PACKAGING/nsis/setup.nsi")"
      mv -f "$SETUP_ASCII" "$SETUP_EXE"
      BUILT=1
      break
    fi
  done
fi

echo "完成。输出目录: $DIST"
