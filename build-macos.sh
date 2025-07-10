#!/usr/bin/env bash

# -------------------------------------------
# Suppress printing of error messages
# exec 2>/dev/null

# Stop on first error
set -o errexit
# Set trap on ERR to be inherited by shell functions
set -o errtrace

# Trap errors
trap 'echo Error at line: $LINENO' ERR
# -------------------------------------------

usage() {
    echo "$(basename $0) [--sign] [--notarize]"
    exit
}

get_opts() {
    while [[ $# -gt 0 ]]; do
        key="$1"
        case $key in
        -s | --sign)
            shift
            SIGN="SIGN"
            ;;
        -n | --notarize)
            shift
            NOTARIZE="NOTARIZE"
            ;;
        -h | --help)
            usage
            ;;
        *)
            usage
            ;;
        esac
    done
}

get_opts $*
if [ $# -gt 2 ]; then
    echo "Too many arguments"
    usage
fi

# -------------------------------------------
cargo fmt
# -------------------------------------------
cargo check --profile dev
cargo clippy --profile dev
cargo build --profile dev --bin treehouse
# -------------------------------------------
cargo check --profile release
cargo clippy --profile release
cargo build --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
cargo doc --profile release --document-private-items --no-deps --workspace
# -------------------------------------------

# -------------------------------------------
EXPORT_PATH="./target/release/bundle/osx"
PRODUCT_NAME="TreeHouse"
APP="${PRODUCT_NAME}.app"
ZIP="${PRODUCT_NAME}.zip"
DMG="${PRODUCT_NAME}.dmg"
NOTARIZATION_RESPONSE_APP="${PRODUCT_NAME}_APP_NotarizationResponse.plist"
NOTARIZATION_RESPONSE_DMG="${PRODUCT_NAME}_DMG_NotarizationResponse.plist"
BUNDLE_PATH="${EXPORT_PATH}/${APP}/Contents"
# -------------------------------------------

# -------------------------------------------
rm -rf "${EXPORT_PATH}"
# -------------------------------------------

# -------------------------------------------
cargo install cargo-bundle 2>/dev/null
cargo-bundle bundle --profile release --bin treehouse
# -------------------------------------------

# -------------------------------------------
FILE="${BUNDLE_PATH}/Info.plist"
TOTAL_LINES=$(wc -l <"${FILE}")
head -n $((TOTAL_LINES - 2)) "${FILE}" >"Info.plist.tmp" && mv "Info.plist.tmp" "${FILE}"
cat "./resources/macos/info_plist_tail.txt" >>"${FILE}"
cp "./resources/macos/"*".icns" "${BUNDLE_PATH}/Resources/"
# -------------------------------------------

cd "${EXPORT_PATH}"
xattr -rc "${APP}"
if [[ -n "${SIGN}" ]]; then
    if [[ -n "${SIGNING_IDENTITY}" ]]; then
        echo -e "\nSigning identity: ${SIGNING_IDENTITY}"
        codesign --sign "${SIGNING_IDENTITY}" --options runtime "${APP}"
        if [[ -n "${NOTARIZE}" ]]; then
            zip -r "${ZIP}" "${APP}"
            if [[ -n "${NOTARYTOOL_KEYCHAIN_PROFILE}" ]]; then
                xcrun notarytool submit \
                    --keychain-profile "${NOTARYTOOL_KEYCHAIN_PROFILE}" \
                    --verbose "${ZIP}" \
                    --wait \
                    --timeout 2h \
                    --output-format plist >"${NOTARIZATION_RESPONSE_APP}"

                return_code=$?

                if [ $return_code -eq 0 ]; then
                    xcrun stapler staple "${APP}"
                    xcrun stapler validate "${APP}"

                    rm -rf "${ZIP}"

                    mkdir -p "${PRODUCT_NAME}"
                    mv -v "${APP}" "${PRODUCT_NAME}"

                    /usr/bin/hdiutil create -srcfolder "${PRODUCT_NAME}" -format UDBZ "${DMG}"

                    xcrun notarytool submit \
                        --keychain-profile "${NOTARYTOOL_KEYCHAIN_PROFILE}" \
                        --verbose "${DMG}" \
                        --wait \
                        --timeout 2h \
                        --output-format plist >"${NOTARIZATION_RESPONSE_DMG}"

                    return_code=$?

                    if [ $return_code -eq 0 ]; then
                        xcrun stapler staple "${DMG}"
                        xcrun stapler validate "${DMG}"
                    fi
                fi
            fi
        fi
    else
        codesign --deep --force --sign - "${APP}"
    fi
fi
# -------------------------------------------
