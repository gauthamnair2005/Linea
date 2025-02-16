# ProcyonCLS Extended Kernel

import os
import sys
import time
import kernel
import getpass
import hashlib
import pyfiglet
import sqlite3

def splashScreen(name, ver):
    kernel.clrscr()
    prettyPrint(name)
    kernel.println(ver)
    time.sleep(2)
    kernel.clrscr()

def prettyPrint(param):
    kernel.println(pyfiglet.figlet_format(param))

def printHeader(header):
    color = "\033[96m"
    reset = "\033[0m"
    print(f"{color}▓▒ {header} ▒░{reset}")

def hash_password(password):
    return hashlib.sha256(password.encode()).hexdigest()

def securePass(display):
    return hash_password(getpass.getpass(display))

def admin(username, display = "Enter Password : "):
    password = getpass.getpass(display)
    conn = sqlite3.connect('configuration.db')
    cursor = conn.cursor()
    cursor.execute(f'SELECT password FROM users WHERE username = "{username}"')
    if cursor.fetchone()[0] == password:
        return True
    else:
        return False
        
def textBrowser(url):
    import requests
    from bs4 import BeautifulSoup
    try:
        response = requests.get(url, verify=True)
        soup = BeautifulSoup(response.content, 'html.parser')
        text = soup.get_text()
        kernel.println(text)
    except:
        kernel.printError("Error fetching page")