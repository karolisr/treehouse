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
DMG="${PRODUCT_NAME}.dmg"
ZIP="${PRODUCT_NAME}.zip"
NOTARIZATION_RESPONSE_APP_PATH="${EXPORT_PATH}/${PRODUCT_NAME}_APP_NotarizationResponse.plist"
NOTARIZATION_RESPONSE_DMG_PATH="${EXPORT_PATH}/${PRODUCT_NAME}_DMG_NotarizationResponse.plist"
NOTARIZATION_RESPONSE_PKG_PATH="${EXPORT_PATH}/${PRODUCT_NAME}_PKG_NotarizationResponse.plist"
APP_PATH="${EXPORT_PATH}/${APP}"
DMG_PATH="${EXPORT_PATH}/${DMG}"
ZIP_PATH="${EXPORT_PATH}/${ZIP}"
BUNDLE_PATH="${APP_PATH}/Contents"
# -------------------------------------------

# -------------------------------------------
rm -rf "${EXPORT_PATH}"
# -------------------------------------------

# -------------------------------------------
cargo install cargo-bundle 2>/dev/null
cargo-bundle bundle --profile release --package treehouse --bin treehouse
# -------------------------------------------

# -------------------------------------------
FILE="${BUNDLE_PATH}/Info.plist"
TOTAL_LINES=$(wc -l <"${FILE}")
head -n $((TOTAL_LINES - 2)) "${FILE}" >"Info.plist.tmp" && mv "Info.plist.tmp" "${FILE}"
cat "./resources/macos/info_plist_tail.txt" >>"${FILE}"
cp "./resources/macos/"*".icns" "${BUNDLE_PATH}/Resources/"
# -------------------------------------------

APP_PLIST="${BUNDLE_PATH}/Info.plist"
VERSION=$(/usr/libexec/PlistBuddy -c "Print :CFBundleShortVersionString" "${APP_PLIST}")
IDENTIFIER=$(/usr/libexec/PlistBuddy -c "Print :CFBundleIdentifier" "${APP_PLIST}")

PKG_PATH="${EXPORT_PATH}/${PRODUCT_NAME} v${VERSION}.pkg"

xattr -rc "${APP_PATH}"
if [[ -n "${SIGN}" ]]; then
    if [[ -n "${SIGNING_IDENTITY_APPLICATION}" ]]; then
        echo -e "\nSigning identity: ${SIGNING_IDENTITY_APPLICATION}"
        codesign --sign "${SIGNING_IDENTITY_APPLICATION}" --options runtime "${APP_PATH}"

        if [[ -n "${NOTARIZE}" ]]; then
            zip -r "${ZIP_PATH}" "${APP_PATH}"
            if [[ -n "${NOTARYTOOL_KEYCHAIN_PROFILE}" ]]; then
                xcrun notarytool submit \
                    --keychain-profile "${NOTARYTOOL_KEYCHAIN_PROFILE}" \
                    --verbose "${ZIP_PATH}" \
                    --wait \
                    --timeout 2h \
                    --output-format plist >"${NOTARIZATION_RESPONSE_APP_PATH}"

                return_code=$?

                if [ $return_code -eq 0 ]; then
                    xcrun stapler staple "${APP_PATH}"
                    xcrun stapler validate "${APP_PATH}"

                    # ---------------------------------------------------------
                    pkgbuild --component "${APP_PATH}" \
                             --identifier "${IDENTIFIER}" \
                             --version "${VERSION}" \
                             --ownership preserve \
                             --install-location "/Applications" \
                             --sign "${SIGNING_IDENTITY_INSTALLER}" \
                             "${PKG_PATH}"

                    xcrun notarytool submit \
                        --keychain-profile "${NOTARYTOOL_KEYCHAIN_PROFILE}" \
                        --verbose "${PKG_PATH}" \
                        --wait \
                        --timeout 2h \
                        --output-format plist >"${NOTARIZATION_RESPONSE_PKG_PATH}"

                    return_code=$?

                    if [ $return_code -eq 0 ]; then
                    xcrun stapler staple "${PKG_PATH}"
                    xcrun stapler validate "${PKG_PATH}"
                    fi
                    # ---------------------------------------------------------
                    rm -rf "${ZIP_PATH}"
                    # ---------------------------------------------------------
                    # mkdir -p "${EXPORT_PATH}/${PRODUCT_NAME}"
                    # mv -v "${APP_PATH}" "${EXPORT_PATH}/${PRODUCT_NAME}"

                    # /usr/bin/hdiutil create -srcfolder "${EXPORT_PATH}/${PRODUCT_NAME}" -format UDBZ "${DMG_PATH}"

                    # xcrun notarytool submit \
                    #     --keychain-profile "${NOTARYTOOL_KEYCHAIN_PROFILE}" \
                    #     --verbose "${DMG_PATH}" \
                    #     --wait \
                    #     --timeout 2h \
                    #     --output-format plist >"${NOTARIZATION_RESPONSE_DMG_PATH}"

                    # return_code=$?

                    # if [ $return_code -eq 0 ]; then
                    #     xcrun stapler staple "${DMG_PATH}"
                    #     xcrun stapler validate "${DMG_PATH}"
                    # fi
                    # ---------------------------------------------------------
                    rm -rf "${APP_PATH}"
                fi
            fi
        fi
    else
        codesign --deep --force --sign - "${APP_PATH}"
    fi
fi
# -------------------------------------------
