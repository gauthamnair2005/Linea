import kernel
import sys
import ekernel
import sqlite3
import time

def main():
    if len(sys.argv) >= 2:
        if sys.argv[1] == "KRNL_0.7":
            ekernel.splashScreen("ProcyonCLS Database", "Version 0.7 Munnar Engine SQLite3")
            ekernel.printHeader("ProcyonDB")
            prompt = ""
            while prompt != "exit":
                prompt = input("ProcyonDB> ")
                if prompt == "exit":
                    break
                elif prompt == "create database":
                    database = input("Enter database name: ")
                    if database == "exit":
                        break
                    database = "databases/" + database + ".db"
                    connection = sqlite3.connect(database)
                    cursor = connection.cursor()
                    kernel.println("Database created successfully")
                elif prompt == "help":
                    kernel.println("ProcyonDB Engine SQLite3")
                    kernel.println("Commands: help, exit")
                else:
                    try:
                        cursor.execute(prompt)
                        connection.commit()
                        kernel.println("Command executed successfully")
                    except:
                        kernel.printError("Invalid command or an error occurred")
        else:
            kernel.printError("This version of database client is incompatible with current version of ProcyonCLS")
    else:
        kernel.printError("OS Scope Error")

if __name__ == "__main__":
    main()