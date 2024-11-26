import kernel
import sys
import ekernel
import time

def main():
    if len(sys.argv) >= 2:
        if sys.argv[1] == "KRNL_0.7":
            ekernel.splashScreen("ProcyonCLS Notes", "Version 0.7 Munnar")
            ekernel.printHeader("Notes")
            filename = input("Enter filename : ").strip()
            filename = "notes/" + filename
            accessMode = input("Enter access mode (r/w/a/r+) : ").strip()
            try:
                if accessMode == "r":
                    with open(filename, "r") as file:
                        kernel.println(file.read())
                elif accessMode == "w":
                    with open(filename, "w") as file:
                        file.write(input("Enter text : "))
                elif accessMode == "a":
                    with open(filename, "a") as file:
                        file.write(input("Enter text : "))
                elif accessMode in ["rw", "ra"]:
                    with open(filename, "r+") as file:
                        kernel.println(file.read())
                        file.write(input("Enter text : "))
                else:
                    kernel.printError("Invalid access mode")
            except:
                kernel.printError("Error accessing file:")
        else:
            kernel.printError("This version of notes is incompatible with current version of ProcyonCLS")
    else:
        kernel.printError("OS Scope Error")

if __name__ == "__main__":
    main()