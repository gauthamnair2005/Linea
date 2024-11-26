# Kernel for Procyon Command Line System

import sys
import os
import time
from datetime import datetime

def log_error(message):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    with open("debug.log", "a") as file:
        file.write(f"[{timestamp}] {message}\n")

try:
    def main():
        if len(sys.argv) == 2:
            if sys.argv[1] == "BTLDR_0.7":
                os.system("cls" if sys.platform == "win32" else "clear")
                print("Procyon Neo Kernel v0.7")
                print("..............")
                print("Loading Kernel modules...")
                time.sleep(1)
                print("PDD", end=" ", flush=True)
                time.sleep(5)
                print("\033[92m[Done]\033[0m")
                print("API", end=" ", flush=True)
                time.sleep(5)
                print("\033[92m[Done]\033[0m")
                print("Kernel Loaded Successfully!")
                print("Booting ProcyonCLS...")
                time.sleep(2)
                if os.path.exists("shell.py"):
                    log_error("Kernel Loaded Successfully")
                    os.system("python3 shell.py KRNL_0.7")
                else:
                    os.system("cls" if sys.platform == "win32" else "clear")
                    print("Kernel Panic : OS error")
                    print("Technical Details : ")
                    print(" Error Code : 0x0003")
                    print(" Error Description : OS not linked to Kernel")
                    log_error("OS not linked to Kernel")
                    sys.exit(1)
            else:
                os.system("cls" if sys.platform == "win32" else "clear")
                print("Kernel Panic : Bootloader error")
                print("Technical Details : ")
                print(" Error Code : 0x0001")
                print(" Error Description : Incompatible version reported by Bootloader")
                print(f" Reported {sys.argv[1]} as opposed to BTLDR_0.7")
                log_error("Incompatible version reported by Bootloader")
                sys.exit(1)
        else:
            os.system("cls" if sys.platform == "win32" else "clear")
            print("Kernel Panic : Bootloader error")
            print("Technical Details : ")
            print(" Error Code : 0x0002")
            print(" Error Description : Kernel invoked without Bootloader")
            log_error("Kernel invoked without Bootloader")
            sys.exit(1)

    if __name__ == "__main__":
        main()

except:
    log_error("Unhandled exception occurred")
    sys.exit(1)


# Kernel APIs

def shutDown():
    println("Shutting down...")
    return sys.exit(0)

def reboot():
    println("Rebooting...")
    os.execv(sys.executable, ['python3', 'bootload.py'])

def getVersion():
    return "KRNL_0.7"

def getBuild():
    return "2024.11.26.1209"

def getAuthor():
    return "Gautham Nair"

def getCompany():
    return "Procyonis Computing"

def getLicense():
    return "GNU GPL v3.0"

def getKernelName():
    return "ProcyonCLS Kernel"

def getCodeName():
    return "Munnar"

def getRelease():
    return "Preliminary Release"

def printError(param):
    print(f"\033[91m{param}\033[0m")

def printWarning(param):
    print(f"\033[93m{param}\033[0m")

def printSuccess(param):
    print(f"\033[92m{param}\033[0m")

def printInfo(param):
    print(f"\033[94m{param}\033[0m")

def println(param):
    print(param)

def clrscr():
    os.system("cls" if sys.platform == "win32" else "clear")

def bsod(error, msg):
    os.system("cls" if sys.platform == "win32" else "clear")
    print("\033[44m\033[97m")
    print(""*1000000000)
    os.system("cls" if sys.platform == "win32" else "clear")        
    print("Kernel Panic : System Failure")
    print("Technical Details : ")
    print(f" Error Code : {error}")
    print(f" Error Description : {msg}")
    print("\033[0m")
    time.sleep(5)
    sys.exit(1)

def callApplication(app, isAdmin = False):
    appResolved = app + ".py KRNL_0.7 " + str(isAdmin)
    os.system(appResolved)