import kernel
import sys
import ekernel
import time
import requests
import zipfile
import shutil
import os

owner = "gauthamnair2005"
repo = "ProcyonCLS"
current_tag_file = "tag.txt"
current_directory = os.getcwd()
db_file = "configuration.db"
protected_dirs = ["databases", "notes"]

def getLatestReleaseTag():
    try:
        url = f"https://api.github.com/repos/{owner}/{repo}/releases/latest"
        response = requests.get(url)
        response.raise_for_status()
        release_info = response.json()
        latest_tag = release_info["tag_name"], release_info["zipball_url"]
        return latest_tag
    except requests.RequestException as e:
        kernel.printError(f"Error fetching latest release: {e}")
        sys.exit(1)

def readCurrentTag():
    if os.path.exists(current_tag_file):
        with open(current_tag_file, "r") as f:
            return f.read().strip()
    return None

def writeCurrentTag(tag):
    with open(current_tag_file, "w") as f:
        f.write(tag)

def downloadRelease(url, dest):
    try:
        response = requests.get(url, stream=True)
        response.raise_for_status()
        with open(dest, 'wb') as f:
            for chunk in response.iter_content(chunk_size=8192):
                f.write(chunk)
    except requests.RequestException as e:
        kernel.printError(f"Error downloading release: {e}")
        sys.exit(1)

def extractRelease(zip_path, extract_to):
    try:
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            zip_ref.extractall(extract_to)
    except zipfile.BadZipFile as e:
        kernel.printError(f"Error extracting release: {e}")
        sys.exit(1)

def replaceLocalFiles(extracted_path, target_path):
    if not os.path.exists(extracted_path):
        kernel.printError(f"Extracted path does not exist: {extracted_path}")
        sys.exit(1)
    for item in os.listdir(extracted_path):
        s = os.path.join(extracted_path, item)
        d = os.path.join(target_path, item)
        if os.path.isdir(s):
            if os.path.basename(d) in protected_dirs:
                continue
            if os.path.exists(d):
                shutil.rmtree(d)
            shutil.copytree(s, d)
        else:
            if os.path.basename(d) == db_file:
                continue
            shutil.copy2(s, d)

def main():
    if len(sys.argv) >= 2:
        if sys.argv[1] != None:
            ekernel.splashScreen("ProcyonCLS Updater", "Version 0.7 Munnar")
            ekernel.printHeader("ProcyonCLS Updater")
            kernel.println("Checking for updates...")
            time.sleep(2)
            latest_tag, zip_url = getLatestReleaseTag()
            current_tag = readCurrentTag()
            if latest_tag != current_tag:
                kernel.printInfo(f"Update available: {current_tag} -> {latest_tag}")
                confirm = input("Do you want to update? (y/n) : ").strip()
                if confirm.lower() != "y":
                    kernel.printWarning("Update cancelled by user")
                    sys.exit(0)
                kernel.println("Updating ProcyonCLS...")
                zip_path = os.path.join(current_directory, "latest_release.zip")
                temp_extract_path = os.path.join(current_directory, "temp")
                downloadRelease(zip_url, zip_path)
                extractRelease(zip_path, temp_extract_path)

                extracted_folder_name = None
                for item in os.listdir(temp_extract_path):
                    if os.path.isdir(os.path.join(temp_extract_path, item)):
                        extracted_folder_name = item
                        break

                if extracted_folder_name:
                    extracted_path = os.path.join(temp_extract_path, extracted_folder_name)
                    replaceLocalFiles(extracted_path, current_directory)
                    writeCurrentTag(latest_tag)
                    kernel.printSuccess("Update completed successfully!")
                    time.sleep(1)
                    kernel.println("Rebooting...")
                    kernel.reboot()
                else:
                    kernel.printError("No extracted folder found.")
                shutil.rmtree(temp_extract_path)
                os.remove(zip_path)
            else:
                kernel.printSuccess("You're up to date!")
                time.sleep(1)
                kernel.println("Shutting down..")
        else:
            kernel.printError("This version of updater is incompatible with the current version of ProcyonCLS")
    else:
        kernel.printError("OS Scope Error")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        kernel.bsod("0x0005", "User interrupted execution")
    except Exception as e:
        kernel.bsod("0x0006", f"Error : {e}")