import kernel
import sys
import os
import pyfiglet
import time
import getpass
import ekernel
import sqlite3

def initialize_db():
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute('''CREATE TABLE IF NOT EXISTS users (
                        username TEXT PRIMARY KEY,
                        password TEXT NOT NULL,
                        first_name TEXT,
                        last_name TEXT,
                        age INTEGER,
                        other_details TEXT)''')
    conn.commit()
    conn.close()

def add_user(username, password, first_name, last_name, age, other_details):
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute('INSERT INTO users (username, password, first_name, last_name, age, other_details) VALUES (?, ?, ?, ?, ?, ?)', 
                   (username, password, first_name, last_name, age, other_details))
    conn.commit()
    conn.close()

def get_user(username):
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute('SELECT * FROM users WHERE username = ?', (username,))
    user = cursor.fetchone()
    conn.close()
    return user

def get_name(username):
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute('SELECT first_name FROM users WHERE username = ?', (username,))
    name = cursor.fetchone()[0]
    conn.close()
    return name

def update_user(username, field, value):
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute(f'UPDATE users SET {field} = ? WHERE username = ?', (value, username))
    conn.commit()
    conn.close()

def delete_user(username):
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute('DELETE FROM users WHERE username = ?', (username,))
    conn.commit()
    conn.close()

def create_user_applet():
    ekernel.printHeader("User Creation")
    username = input("Enter Username: ").strip()
    password = getpass.getpass("Enter Password: ").strip()
    first_name = input("Enter First Name: ").strip()
    last_name = input("Enter Last Name: ").strip()
    age = int(input("Enter Age: ").strip())
    other_details = input("Enter Other Details: ").strip()
    add_user(username, password, first_name, last_name, age, other_details)
    kernel.printSuccess("User Created Successfully!")
    kernel.printWarning("Please wait..")
    time.sleep(5)
    prompt(first_name, username)

def prompt(user, username):
    kernel.clrscr()
    ekernel.prettyPrint(f"Welcome, {user}")
    time.sleep(5)
    kernel.clrscr()
    ekernel.printHeader("ProcyonCLS Shell")
    kernel.printWarning("/!\\ This is a preliminary release..!!")
    while True:
        prmpt = input(f"{username}@ProcyonCLS:~$ ").strip()
        if prmpt == "exit" or prmpt == "shutdown":
            kernel.shutDown()
        elif prmpt == "reboot":
            kernel.reboot()
        elif prmpt == "delete":
            fileFolder = input("Enter file name to delete : ").strip()
            if os.path.exists(fileFolder):
                try:
                    os.remove(fileFolder)
                    kernel.printSuccess(f"{fileFolder} deleted successfully!")
                except Exception as e:
                    kernel.printError(f"Error deleting file: {e}")
            else:
                kernel.printError(f"{fileFolder} not found")
        elif prmpt == "mkdir":
            folder = input("Enter folder name : ").strip()
            if sys.platform == "win32" and folder.lower() in ["con", "prn", "aux", "nul"] + [f"com{i}" for i in range(1, 10)] + [f"lpt{i}" for i in range(1, 10)]:
                kernel.bsod("0x0007", "Attempted to create system file")
            else:
                try:
                    os.mkdir(folder)
                    kernel.printSuccess(f"{folder} created successfully!")
                except Exception as e:
                    kernel.printError(f"Error creating folder: {e}")
        elif prmpt == "notes":
            kernel.callApplication("notes", isAdmin=False)
        elif prmpt == "database":
            kernel.callApplication("database", isAdmin=False)
        elif prmpt == "linea":
            kernel.println("Coming Soon!")
        elif prmpt == "python":
            os.system("python3")
        elif prmpt == "clsupdate":
            if ekernel.admin(username):
                os.execv(sys.executable, ['python3', 'updater.py', 'KRNL_0.5'])
            else:
                kernel.printError("Admin access denied, updater needs admin access to run!")
        elif prmpt == "security":
            if ekernel.admin(username):
                kernel.callApplication("security", isAdmin=True)
            else:
                kernel.printError("Admin access denied, security needs admin access to run!")
        elif prmpt in ["dir", "ls"]:
            kernel.println(os.listdir())
        elif prmpt == "ver":
            ekernel.printHeader("Version Information")
            kernel.printInfo(f"Version : {kernel.getVersion()}")
            kernel.printInfo(f"Code Name : {kernel.getCodeName()}")
            kernel.printInfo(f"Release : {kernel.getRelease()}")
        elif prmpt == "info":
            ekernel.printHeader("Software Information")
            kernel.printInfo(f"Version : {kernel.getVersion()}")
            kernel.printInfo(f"Build : {kernel.getBuild()}")
            kernel.printInfo(f"Author : {kernel.getAuthor()}")
            kernel.printInfo(f"Company : {kernel.getCompany()}")
            kernel.printInfo(f"License : {kernel.getLicense()}")
            kernel.printInfo(f"Name : {kernel.getKernelName()}")
            kernel.printInfo(f"Code Name : {kernel.getCodeName()}")
            kernel.printInfo(f"Release : {kernel.getRelease()}")
        elif prmpt in ["calc", "calculator", "eval", "evaluator"]:
            try:
                kernel.callApplication("evaluator", isAdmin=False)
            except:
                kernel.printError(f"Error running evaluator")
        elif prmpt == "date":
            kernel.println(time.strftime("%d/%m/%Y"))
        elif prmpt == "time":
            kernel.println(time.strftime("%H:%M:%S"))
        elif prmpt == "datetime":
            kernel.println(time.strftime("%d/%m/%Y %H:%M:%S"))
        elif prmpt in ["reset password", "reset-password"]:
            ekernel.printHeader("Reset Password")
            if ekernel.admin("Enter old password : "):
                password = getpass.getpass("Enter new password : ").strip()
                update_user(user, 'password', password)
                kernel.printSuccess("Password reset successfully!")
            else:
                kernel.printError("Admin access denied")
        elif prmpt.startswith("update "):
            parts = prmpt.split()
            if len(parts) == 3:
                field, value = parts[1], parts[2]
                if field in ['username', 'first_name', 'last_name', 'age', 'other_details']:
                    update_user(user, field, value)
                    kernel.printSuccess(f"{field} updated successfully!")
                else:
                    kernel.printError("Invalid field")
            else:
                kernel.printError("Usage: update <field> <value>")
        elif prmpt == "create user":
            create_user_applet()
        elif prmpt == "help":
            ekernel.printHeader("Help")
            kernel.printInfo("Available commands :")
            kernel.printInfo("help - Display this help message")
            kernel.printInfo("exit - Exit the shell")
            kernel.printInfo("bsod - Invoke a Blue Screen of Death")
            kernel.printInfo("run <application> - Run a 3rd party application")
            kernel.printInfo("admin <application> - Run a 3rd party application with admin privileges")
            kernel.printInfo("clrscr - Clear the screen")
            kernel.printInfo("eval - Open the evaluator")
            kernel.printInfo("date - Display the current date")
            kernel.printInfo("time - Display the current time")
            kernel.printInfo("datetime - Display the current date and time")
            kernel.printInfo("reset password - Reset the user password")
            kernel.printInfo("update <field> <value> - Update user details")
            kernel.printInfo("create user - Create a new user")
            kernel.printInfo("ver - Display OS version information")
            kernel.printInfo("info - Display OS information")
            kernel.printInfo("notes - Open the notes application")
            kernel.printInfo("dir/ls - List files and folders in the current directory")
            kernel.printInfo("mkdir - Create a new folder")
            kernel.printInfo("delete - Delete a file")
            kernel.printInfo("reboot - Reboot the system")
            kernel.printInfo("shutdown - Shutdown the system")
        elif prmpt == "logout":
            break
        elif prmpt == "delete user":
            if ekernel.admin(username):
                delete_user(username)
                kernel.printSuccess("User deleted successfully!")
                break
            else:
                kernel.printError("Admin access denied")
        elif prmpt == "bsod":
            kernel.bsod("0x0004", "User invoked BSOD")
        elif prmpt.startswith("run "):
            try:
                kernel.callApplication(prmpt[4:], isAdmin=False)
            except Exception as e:
                kernel.printError(f"Error running 3rd party application: {e}")
        elif prmpt.startswith("admin "):
            if ekernel.admin(username):
                if prmpt[6:] in ["bootload", "kernel", "shell", "ekernel"]:
                    kernel.printError("Cannot run system files")
                else:
                    try:
                        kernel.callApplication(prmpt[6:], isAdmin=True)
                    except Exception as e:
                        kernel.printError(f"Error running 3rd party application: {e}")
            else:
                kernel.printError("Admin access denied")
        elif prmpt == "clrscr":
            kernel.clrscr()
        elif prmpt == "" or prmpt.isspace():
            continue
        else:
            kernel.printError("Command not found")

def main():
    initialize_db()
    if len(sys.argv) == 2:
        if sys.argv[1] == "KRNL_0.7":
            os.system("cls" if sys.platform == "win32" else "clear")
            print(pyfiglet.figlet_format("֎ ProcyonCLS", font="slant", justify="center"))
            print(pyfiglet.figlet_format("Preliminary Release", font="slant", justify="center"))
            print("\n\n\nCopyright © 2024, Procyonis Computing\n\n\nStarting...")
            for _ in range(10):
                print("═", end="", flush=True)
                time.sleep(0.5)
            for _ in range(100):
                print("═", end="", flush=True)
                time.sleep(0.1)
            for _ in range(3):
                print("═", end="", flush=True)
                time.sleep(0.2)
            time.sleep(2)
            kernel.clrscr()
            kernel.println("Welcome")
            time.sleep(1.5)
            kernel.clrscr()
            conn = sqlite3.connect('configuration.db')
            cursor = conn.cursor()
            cursor.execute('SELECT COUNT(*) FROM users')
            user_count = cursor.fetchone()[0]
            conn.close()
            if user_count == 0:
                create_user_applet()
            else:
                while True:
                    ekernel.printHeader("Login")
                    username = input("Enter Username: ").strip()
                    password = getpass.getpass("Enter Password: ").strip()
                    user_data = get_user(username)
                    if user_data and user_data[1] == password:
                        kernel.printSuccess("Login Successful!")
                        kernel.printWarning("Please wait..")
                        time.sleep(5)
                        prompt(get_name(username), username)
                        break
                    else:
                        kernel.printError("Login Failed!")
        else:
            print("OS Error : Kernel version mismatch")
            print(f"Expected KRNL_0.7, got {sys.argv[1]}")
            sys.exit(1)
    else:
        print("OS Error : Shell needs kernel to run")
        sys.exit(1)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        kernel.bsod("0x0005", "User interrupted execution")
    except Exception as e:
        kernel.bsod("0x0006", f"Error : {e}")