import sys
import kernel
import ekernel

def main():
    if len(sys.argv) == 3:
        if sys.argv[1] == "KRNL_0.7":
            if sys.argv[2] == "False":
                ekernel.splashScreen("App Sample for ProcyonCLS", "Version 0.7 Compatible App 0.1")
                ekernel.printHeader("Sample")
                kernel.println("Hello, World!")
                ekernel.prettyPrint("Hello, World!")
                kernel.println("Admin Status : False")
                # Need new admin access from admin()
                kernel.printInfo("This is information text")
                kernel.printError("This is error text")
                kernel.printWarning("This is warning text")
                kernel.printSuccess("This is success text")
                kernel.println("This is normal text")
            else:
                # Admin Access True
                ekernel.splashScreen("App Sample for ProcyonCLS", "Version 0.7 Compatible App 0.1")
                ekernel.printHeader("Sample")
                kernel.println("Hello, World!")
                ekernel.prettyPrint("Hello, World!")
                kernel.println("Admin Status : True")
                # Already run as admin, no need for new request
                kernel.printInfo("This is information text")
                kernel.printError("This is error text")
                kernel.printWarning("This is warning text")
                kernel.printSuccess("This is success text")
                kernel.println("This is normal text")
        else:
            kernel.printError("Incompatible kernel")
    else:
        kernel.printError("OS Scope Error")

if __name__ == "__main__":
    main()