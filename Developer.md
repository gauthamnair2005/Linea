# ProcyonCLS - Command Line System

## Developing Applications for ProcyonCLS

### Introduction

Developing applications for ProcyonCLS is a simple process. ProcyonCLS is a command line system that is designed to be easy to use and easy to develop for. This document will guide you through the process of developing applications for ProcyonCLS.

### Getting Started

To get started developing applications for ProcyonCLS, you will need to have a basic understanding of the command line and how to use it. You will also need to have a basic understanding of the ProcyonCLS command line system.

### Creating a New Application

Applications for ProcyonCLS are written in Python. To create a new application, create a new Python file with a `.py` extension. This file will contain the code for your application.

#### Writing Code

ProcyonCLS provides a Kernel API and an Extended Kernel API, which you can use to interact with the command line system. You can import these APIs into your application using the following code:

```python
import kernel
import ekernel
import sys

def main():
    if len(sys.argv) == 2:
        if sys.argv[1] == "KRNL_0.7":
            kernel.println("Hello, World!")
            ekernel.prettyPrint("Hello, World!")
        else:
            kernel.printError("Incompatible kernel")
    else:
        kernel.printError("OS Scope Error")

if __name__ == "__main__":
    main()
```

The above code is a simple example of a ProcyonCLS application. It imports the Kernel API and the Extended Kernel API, and prints "Hello, World!" to the command line, both in plain text and in a pretty format.

#### Understanding the Code

* `import kernel` - This is the main kernel as well as base API provider for ProcyonCLS and extended kernel. In this code, the `println()` and `printError()` are provided by the kernel API.

* `import ekernel` - This is the extended kernel API provider for ProcyonCLS. In this code, the `prettyPrint()` is provided by the extended kernel API.

* `import sys` - This is the system module for Python. It provides access to some variables used or maintained by the interpreter and to functions that interact strongly with the interpreter.

* `def main():` - This is the main function of the application. It checks the command line arguments and prints "Hello, World!" to the command line.

* `if len(sys.argv) == 2:` - This checks if there are two command line arguments.

* `if sys.argv[1] == "KRNL_0.7":` - This checks if the second command line argument is `KRNL_0.7`.

* `kernel.println("Hello, World!")` - This prints "Hello, World!" to the command line.

* `ekernel.prettyPrint("Hello, World!")` - This prints "Hello, World!" to the command line in a pretty format.

* `else:` - This is the else statement for the `if sys.argv[1] == "KRNL_0.7":` statement.

* `kernel.printError("Incompatible kernel")` - This prints an error message to the command line.

* `else:` - This is the else statement for the `if len(sys.argv) == 2:` statement.

* `kernel.printError("OS Scope Error")` - This prints an error message to the command line.

* `if __name__ == "__main__":` - This checks if the script is being run as the main program.

* `main()` - This calls the main function.

### Running the Application

To run this application, you need to first logon to ProcyonCLS and then type `run <yourapplication>` in the command line. For example, if your application is named `hello.py`, you would type `run hello` in the command line.

### Conclusion

This document has provided an overview of how to develop applications for ProcyonCLS. For more information on developing applications for ProcyonCLS, refer to the ProcyonCLS documentation.