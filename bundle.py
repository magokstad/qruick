
# dx bundle --platform ios --release --device true
# cp Info.plist ./target/dx/qruick/release/ios/Qruick.app
# cp Qruick_Test_Profile.mobileprovision ./target/dx/qruick/release/ios/Qruick.app/embedded.mobileprovision
# xattr -cr .
# codesign --sign "Magnus Økstad" --entitlements ./entitlements.plist -f ./target/dx/qruick/release/ios/Qruick.app

import subprocess
import shutil
import os
import sys
import argparse

def run_command(cmd):
    """Run a shell command and raise if it fails."""
    print(f"Running: {cmd}")
    result = subprocess.run(cmd, shell=True)
    if result.returncode != 0:
        sys.exit(f"Command failed: {cmd}")

def main():
    parser = argparse.ArgumentParser(description="iOS bundling and signing script")
    parser.add_argument(
        "-i", "--identity",
        default="Magnus Økstad",
        help="Code signing identity (default: 'Magnus Økstad')"
    )
    args = parser.parse_args()

    app_dir = "./target/dx/qruick/release/ios/Qruick.app"

    # 1. Build bundle
    run_command("dx bundle --platform ios --release --device true")

    # 2. Copy Info.plist
    shutil.copy("Info.plist", os.path.join(app_dir, "Info.plist"))

    # 3. Copy provisioning profile
    shutil.copy(
        "Qruick_Test_Profile.mobileprovision",
        os.path.join(app_dir, "embedded.mobileprovision")
    )

    # 4. Clear extended attributes
    run_command("xattr -cr .")

    # 5. Code signing with provided identity
    run_command(
        f'codesign --sign "{args.identity}" --entitlements ./entitlements.plist -f {app_dir}'
    )

if __name__ == "__main__":
    main()
