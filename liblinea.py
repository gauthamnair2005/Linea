import threading
import math
import random
import statistics
import time
import webbrowser
import sys
import os
import requests
import matplotlib as plt
import PyQt6.QtWebEngineCore
import PyQt6.QtWebEngineWidgets
import PyQt6.QtCore

pi = math.pi
e = math.e
tau = math.tau
inf = math.inf
nan = math.nan
rand = random.random
randint = random.randint
randrange = random.randrange
choice = random.choice
shuffle = random.shuffle
seed = random.seed
sqrt = math.sqrt
pow = math.pow
exp = math.exp
lVar = {}
lAdr = {}
lAct = {}
workerStore = {}
floatNotCase = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "'", '"', ",", "/", "\\", "<", ">", ";", ":", "[", "]", "{", "}", "-", "_", "+", "=", "(", ")", "!", "@", "#", "$", "%", "^", "&", "*", "~", "`", "|"]
operators = ["+", "-", "*", "/", "%", "^", "(", ")"]

def displayLSP(param):
    return param

def forLoopRange(param):
    res = ""
    for i in range(param):
        res += str(i)
    return res

def ver():
    return "Linea 1.8 'Mishka' LSP/SLS 0.1"

def getInput():
    return input()

def getLineaVersion():
    return "Linea 1.8 'Mishka'"

def getLineaVersionNumber():
    return "1.8"

def getLineaVersionCode():
    return "Mishka"

def getLSPVersion():
    return "LSP/SLS 0.1"

def getLSPVersionNumber():
    return "0.1"

def getSLSVersion():
    return getLSPVersion()

def getSLSVersionNumber():
    return getLSPVersionNumber()

def removeTagsFromLSPCode(LSPCode):
    if "<?lsp" not in LSPCode or "?>" not in LSPCode:
        return "Syntax Error: Missing LSP tags"
    LSPCode = LSPCode.replace("<?lsp", "")
    LSPCode = LSPCode.replace("?>", "")
    LSPCode = LSPCode.strip()
    return LSPCode

def breakPhraseToWords(param, _LSP_VARIABLES):
    # Split the phrase by the '+' operator
    sepComp = []
    words = param.split("+")
    words = [word.strip() for word in words]
    for word in words:
        if word in _LSP_VARIABLES:  # Check if the word is a variable
            sepComp.append(_LSP_VARIABLES[word])
        elif (word.startswith('"') and word.endswith('"')) or (word.startswith("'") and word.endswith("'")):
            sepComp.append(word[1:-1])  # Remove quotes from string literals
        elif word.isdigit():  # Check if the word is a number
            sepComp.append(word)
        else:
            return f"Syntax Error: Variable '{word}' not found"
    return "".join(sepComp)

def evaluate(param, _LSP_VARIABLES):
    terms = param.split(" ")
    terms = [term.strip() for term in terms]
    for i in range(len(terms)):
        if terms[i] in _LSP_VARIABLES:
            terms[i] = _LSP_VARIABLES[terms[i]]
        elif terms[i].isdigit():
            terms[i] = int(terms[i])
        elif terms[i] in operators:
            continue
        else:
            return f"Syntax Error: Variable '{terms[i]}' not found"
    return eval(" ".join([str(term) for term in terms]))

def runJavaScript(param):
    if param.startswith("get.element.id "):
        return f'<script>document.getElementById("{param[15:]}")</script>'
    elif param.startswith("get.element.id.value "):
        return f'<script>document.getElementById("{param[21:]}").value</script>'
    elif param.startswith("get.element.id.innerHTML "):
        return f'<script>document.getElementById("{param[26:]}").innerHTML</script>'
    elif param.startswith("get.element.id.innerText "):
        return f'<script>document.getElementById("{param[25:]}").innerText</script>'
    elif param.startswith("get.element.class "):
        return f'<script>document.getElementsByClassName("{param[18:]}")</script>'
    elif param.startswith("get.element.tag "):
        return f'<script>document.getElementsByTagName("{param[16:]}")</script>'
    elif param.startswith("get.element.name "):
        return f'<script>document.getElementsByName("{param[17:]}")</script>'
    elif param.startswith("get.element.query "):
        return f'<script>document.querySelector("{param[18:]}")</script>'
    elif param.startswith("get.element.queryAll "):
        return f'<script>document.querySelectorAll("{param[21:]}")</script>'
    elif param.startswith("log "):
        return f'<script>console.log("{param[4:]}")</script>'
    else:
        return f"Syntax Error: Unknown LSP JavaScript '{param}'"
    
def prime(param, line):
    try:
        n = int(param)
        if n > 1:
            for i in range(2, n):
                if n % i == 0:
                    print("False")
                    break
            else:
                print("True")
        else:
            print("False")
    except:
        displayError("L-E11 : Invalid input", line)

def fibonacci(param, line):
    try:
        n = int(param)
        a, b = 0, 1
        for i in range(n):
            a, b = b, a + b
        print(a)
    except:
        displayError("L-E11 : Invalid input", line)

def web(param, line = ""):
    try:
        html_content = param.strip()
        app = PyQt6.QtWidgets.QApplication(["Linea Weblet"])
        PyQt6.QtWebEngineCore.QWebEngineProfile.defaultProfile().setHttpCacheType(
            PyQt6.QtWebEngineCore.QWebEngineProfile.HttpCacheType.MemoryHttpCache
        )
        web_view = PyQt6.QtWebEngineWidgets.QWebEngineView()
        web_view.setHtml(html_content)
        web_view.setWindowTitle("Linea Weblet")  # Set the window title
        web_view.show()
        app.exec()
    except Exception as e:
        print()

def sleep(param):
    time.sleep(int(param))

def permutations(param, line):
    try:
        n, r = param.split(",")
        n = int(n)
        r = int(r)
        result = math.perm(n, r)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def combinations(param, line):
    try:
        n, r = param.split(",")
        n = int(n)
        r = int(r)
        result = math.comb(n, r)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def factorial(param, line):
    try:
        result = math.factorial(int(param))
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def gcd(param, line):
    try:
        a, b = param.split(",")
        a = int(a)
        b = int(b)
        result = math.gcd(a, b)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def lcm(param, line):
    try:
        a, b = param.split(",")
        a = int(a)
        b = int(b)
        result = (a * b) // math.gcd(a, b)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def hcf(param, line):
    try:
        a, b = param.split(",")
        a = int(a)
        b = int(b)
        result = math.gcd(a, b)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def mean(param, line):
    try:
        values = [int(i) for i in param.split(",")]
        result = statistics.mean(values)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def median(param, line):
    try:
        values = [int(i) for i in param.split(",")]
        result = statistics.median(values)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def mode(param, line):
    try:
        values = [int(i) for i in param.split(",")]
        result = statistics.mode(values)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def variance(param, line):
    try:
        values = [int(i) for i in param.split(",")]
        result = statistics.variance(values)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def stdev(param, line):
    try:
        values = [int(i) for i in param.split(",")]
        result = statistics.stdev(values)
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def rungeKutta(param, line):
    try:
        x0, y0, x, h = param.split(",")
        x0 = float(x0)
        y0 = float(y0)
        x = float(x)
        h = float(h)
        n = int((x - x0) / h)
        y = y0
        for i in range(1, n + 1):
            k1 = h * (x0 + i - 1, y)
            k2 = h * (x0 + i - 1 + 0.5 * h, y + 0.5 * k1)
            k3 = h * (x0 + i - 1 + 0.5 * h, y + 0.5 * k2)
            k4 = h * (x0 + i - 1 + h, y + k3)
            y = y + (1 / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
        print(y)
    except:
        displayError("L-E11 : Invalid input", line)

def euler(param, line):
    try:
        x0, y0, x, h = param.split(",")
        x0 = float(x0)
        y0 = float(y0)
        x = float(x)
        h = float(h)
        n = int((x - x0) / h)
        y = y0
        for i in range(1, n + 1):
            y = y + h * (x0 + i - 1, y)
        print(y)
    except:
        displayError("L-E11 : Invalid input", line)

def newtonRaphson(param, line):
    try:
        x0, f, f1 = param.split(",")
        x0 = float(x0)
        f = lambda x: eval(f)
        f1 = lambda x: eval(f1)
        x1 = x0 - f(x0) / f1(x0)
        while abs(x1 - x0) >= 0.0001:
            x0 = x1
            x1 = x0 - f(x0) / f1(x0)
        print(x1)
    except:
        displayError("L-E11 : Invalid input", line)

def bisection(param, line):
    try:
        a, b, f = param.split(",")
        a = float(a)
        b = float(b)
        f = lambda x: eval(f)
        if f(a) * f(b) >= 0:
            displayError("L-E12 : Invalid interval", line)
            return
        c = a
        while (b - a) >= 0.0001:
            c = (a + b) / 2
            if f(c) == 0:
                break
            if f(c) * f(a) < 0:
                b = c
            else:
                a = c
        print(c)
    except:
        displayError("L-E11 : Invalid input", line)

def secant(param, line):
    try:
        x0, x1, f = param.split(",")
        x0 = float(x0)
        x1 = float(x1)
        f = lambda x: eval(f)
        x2 = x1 - (f(x1) * (x1 - x0)) / (f(x1) - f(x0))
        while abs(x2 - x1) >= 0.0001:
            x0 = x1
            x1 = x2
            x2 = x1 - (f(x1) * (x1 - x0)) / (f(x1) - f(x0))
        print(x2)
    except:
        displayError("L-E11 : Invalid input", line)

def trapeziodal(param, line):
    try:
        a, b, f, n = param.split(",")
        a = float(a)
        b = float(b)
        f = lambda x: eval(f)
        n = int(n)
        h = (b - a) / n
        result = f(a) + f(b)
        for i in range(1, n):
            result += 2 * f(a + i * h)
        result *= h / 2
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def simpsons(param, line):
    try:
        a, b, f, n = param.split(",")
        a = float(a)
        b = float(b)
        f = lambda x: eval(f)
        n = int(n)
        h = (b - a) / n
        result = f(a) + f(b)
        for i in range(1, n):
            if i % 2 == 0:
                result += 2 * f(a + i * h)
            else:
                result += 4 * f(a + i * h)
        result *= h / 3
        print(result)
    except:
        displayError("L-E11 : Invalid input", line)

def getMemory():
    print("Memory : " + str(sys.getsizeof(lVar) + sys.getsizeof(lAct) + sys.getsizeof(lAdr)) + " bytes")

def displayWarning(param, line):
    print("\033[93mWarning in line : " + str(line) + " " + param + "\033[0m")

def displayError(param, line):
    print("\033[91mError in line : " + str(line) + " " + param + "\033[0m")

def displayVar():
    print(lVar)

def displayAct():
    print(lAct)

def displayAdr():
    print(lAdr)

def display(param, line):
    param = param.replace("|s|", " ")
    if param.startswith('"') and param.endswith('"') or param.startswith("'") and param.endswith("'"):
        print(param[1:-1])
    elif param in lAct:
        print(lAct[param])
    elif param in lVar:
        print(lVar[param])
    elif param.isdigit():
        print(param)
    elif param in ".":
        print(param)
    elif param == "True" or param == "False" or param == "true" or param == "false":
        print(param)
    elif param == "NULL" or param == "NILL" or param == "null" or param == "nill" or param == "None" or param == "none":
        print(param)
    else:
        try:
            print(eval(param))
        except:
            displayError("L-E3 : Undefined object", line)

def varOps(param, line):
    operator, varName= param.split(" ")
    varName = varName.split(",")
    varName = [i.strip() for i in varName]
    if operator == "+":
        result = 0
        for i in range(0, len(varName)):
            result += float(lVar[varName[i]])
        print(result)
    elif operator == "-":
        result = float(lVar[varName[0]])
        for i in range(1, len(varName)):
            result -= float(lVar[varName[i]])
        print(result)
    elif operator == "*":
        result = 1
        for i in range(0, len(varName)):
            result *= float(lVar[varName[i]])
        print(result)
    elif operator == "/":
        result = float(lVar[varName[0]])
        for i in range(1, len(varName)):
            try:
                result /= float(lVar[varName[i]])
            except:
                displayError("L-E10 : Division by zero", line)
        print(result)
    elif operator == "%":
        result = float(lVar[varName[0]])
        for i in range(1, len(varName)):
            result %= float(lVar[varName[i]])
        print(result)
    elif operator == "^":
        result = float(lVar[varName[0]])
        for i in range(1, len(varName)):
            result **= float(lVar[varName[i]])
        print(result)
    else:
        displayError("L-E2 : Invalid operator", line)

def inputTake(param, line):
    varName, queryText = param.split(" ")
    varName = varName.strip()
    queryText = queryText.strip()
    if queryText in lAct:
        lVar[varName] = input(lAct[queryText]).replace("|s|", " ")
    elif queryText in lVar:
        lVar[varName] = input(lVar[queryText]).replace("|s|", " ")
    elif queryText.isdigit():
        lVar[varName] = input(int(queryText))
    elif queryText.startswith('"') and queryText.endswith('"') or queryText.startswith("'") and queryText.endswith("'"):
        lVar[varName] = input(queryText[1:-1]).replace("|s|", " ")
    else:
        displayError("L-E3 : Undefined object", line)

def clrScr():
    os.system("cls" if sys.platform == "win32" else "clear")

def displayDataType():
    for i in lVar:
        if lVar[i] == lAdr[i]:
            print(f"{i} -> Pointer")
            return
        if type(lVar[i]) == int:
            print(f"{i} -> Integer")
        elif type(lVar[i]) == float:
            print(f"{i} -> Floating Point")
        elif type(lVar[i]) == str:
            print(f"{i} -> String")
        elif type(lVar[i]) == bool:
            print(f"{i} -> Boolean")
        elif type(lVar[i]) == list:
            print(f"{i} -> Array")
        elif type(lVar[i]) == None:
            print(f"{i} -> Null")
        elif type(lVar[i]) == complex:
            print(f"{i} -> Complex Number")
        else:
            print(f"{i} -> Unknown/Other")

def typeCast(param, line):
    newDataType, varName = param.split(" ")
    newDataType = newDataType.strip()
    varName = varName.strip()
    if varName in lVar:
        if newDataType == "int":
            try:
                lVar[varName] = int(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        elif newDataType == "float":
            try:
                lVar[varName] = float(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        elif newDataType == "str":
            try:
                lVar[varName] = str(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        elif newDataType == "bool":
            try:
                lVar[varName] = bool(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        elif newDataType == "list":
            try:
                lVar[varName] = list(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        elif newDataType == "complex":
            try:
                lVar[varName] = complex(lVar[varName])
            except:
                displayError("L-E4 : Type conversion prohibited", line)
        else:
            displayError("L-E5 : Invalid data type", line)

def binView(param, line):
    if param in lAct:
        print(bin(lAct[param]))
    elif param in lVar:
        print(bin(lVar[param]))
    elif param.isdigit():
        print(bin(int(param, line)))
    else:
        displayError("L-E3 : Undefined object", line)

def hexView(param, line):
    if param in lAct:
        print(hex(lAct[param]))
    elif param in lVar:
        print(hex(lVar[param]))
    elif param.isdigit():
        print(hex(int(param, line)))
    else:
        displayError("L-E3 : Undefined object", line)

def octView(param, line):
    if param in lAct:
        print(oct(lAct[param]))
    elif param in lVar:
        print(oct(lVar[param]))
    elif param.isdigit():
        print(oct(int(param, line)))
    else:
        displayError("L-E3 : Undefined object", line)

def typeView(param, line):
    if param in lAct:
        if type(lVar[param]) == int:
            print("Integer")
        elif type(lVar[param]) == float:
            print("Floating Point")
        elif type(lVar[param]) == str:
            print("String")
        elif type(lVar[param]) == bool:
            print("Boolean")
        elif type(lVar[param]) == list:
            print("Array")
        elif type(lVar[param]) == None:
            print("Null")
        elif type(lVar[param]) == complex:
            print("Complex Number")
    elif param in lVar:
        if type(lVar[param]) == int:
            print("Integer")
        elif type(lVar[param]) == float:
            print("Floating Point")
        elif type(lVar[param]) == str:
            print("String")
        elif type(lVar[param]) == bool:
            print("Boolean")
        elif type(lVar[param]) == list:
            print("Array")
        elif type(lVar[param]) == None:
            print("Null")
        elif type(lVar[param]) == complex:
            print("Complex Number")
    elif param in lAdr:
        print("Pointer")
    elif param.isdigit():
        print("Integer")
    elif param == "True" or param == "False" or param == "true" or param == "false" or param == "NULL" or param == "NILL" or param == "null" or param == "nill" or param == "None" or param == "none":
        print("Boolean")
    elif param.startswith('"') and param.endswith('"') or param.startswith("'") and param.endswith("'"):
        print("String")
    else:
        displayError("L-E3 : Undefined object", line)

def var(param, line):
    varName, varValue = param.split("=")
    varName = varName.strip()
    varValue = varValue.strip()
    if varName in lVar:
        displayWarning("L-W1 : Using var to update existing variable is deprecated, use varUpd next time onwards", line)
        lVar[varName] = varValue
    else:
        if varValue.isdigit():
            lVar[varName] = int(varValue)
            lAdr[varName] = id(lVar[varName])
        elif varValue in lVar:
            lVar[varName] = lVar[varValue]
            lAdr[varName] = id(lVar[varName])
        else:
            if varValue.startswith('"') and varValue.endswith('"') or varValue.startswith("'") and varValue.endswith("'"):
                lVar[varName] = varValue[1:-1]
                lAdr[varName] = id(lVar[varName])
            elif varValue == "True" or varValue == "False" or varValue == "true" or varValue == "false":
                lVar[varName] = varValue.upper()
                lAdr[varName] = id(lVar[varName])
            elif varValue == "NULL" or varValue == "NILL" or varValue == "null" or varValue == "nill" or varValue == "None" or varValue == "none":
                lVar[varName] = None
                lAdr[varName] = id(lVar[varName])
            elif "," in varValue:
                value = varValue.strip()
                val0 = value.split(",")
                val0 = [i.strip() for i in val0]
                for i in range(0, len(val0)):
                    if val0[i] in lAct:
                        val0[i] = lAct[val0[i]]
                    elif val0[i] in lVar:
                        val0[i] = lVar[val0[i]]
                    elif val0[i].isdigit():
                        val0[i] = int(val0[i])
                    else:
                        if val0[i].startswith('"') and val0[i].endswith('"') or val0[i].startswith("'") and val0[i].endswith("'"):
                            val0[i] = val0[i][1:-1]
                        else:
                            if val0[i] == "True" or val0[i] == "False" or val0[i] == "true" or val0[i] == "false":
                                val0[i] = val0[i].upper()
                            elif val0[i] == "NULL" or val0[i] == "NILL" or val0[i] == "null" or val0[i] == "nill" or val0[i] == "None" or val0[i] == "none":
                                val0[i] = None
                            else:
                                displayError("L-E3 : Undefined object", line)
                lVar[varName] = val0
                lAdr[varName] = id(lVar[varName])
            elif "." in varValue and varValue not in floatNotCase:
                pointCount = 0
                for i in range(0, len(varValue)):
                    if varValue[i] == ".":
                        pointCount += 1
                    else:
                        pass
                if pointCount == 1:
                    try:
                        lVar[varName] = float(varValue)
                        lAdr[varName] = id(lVar[varName])
                    except:
                        displayError("L-E3 : Undefined object", line)
                else:
                    displayError("L-E3 : Undefined object", line)

def varUpd(param, line):
    varName, varValue = param.split("=")
    varName = varName.strip()
    varValue = varValue.strip()
    if varName in lVar:
        if varValue.isdigit():
            lVar[varName] = int(varValue)
            lAdr[varName] = id(lVar[varName])
        elif varValue in lVar:
            lVar[varName] = lVar[varValue]
            lAdr[varName] = id(lVar[varName])
        else:
            if varValue.startswith('"') and varValue.endswith('"') or varValue.startswith("'") and varValue.endswith("'"):
                lVar[varName] = varValue[1:-1]
                lAdr[varName] = id(lVar[varName])
            elif varValue == "True" or varValue == "False" or varValue == "true" or varValue == "false":
                lVar[varName] = varValue.upper()
                lAdr[varName] = id(lVar[varName])
            elif varValue == "NULL" or varValue == "NILL" or varValue == "null" or varValue == "nill" or varValue == "None" or varValue == "none":
                lVar[varName] = None
                lAdr[varName] = id(lVar[varName])
            elif "," in varValue:
                value = varValue.strip()
                val0 = value.split(",")
                val0 = [i.strip() for i in val0]
                for i in range(0, len(val0)):
                    if val0[i] in lAct:
                        val0[i] = lAct[val0[i]]
                    elif val0[i] in lVar:
                        val0[i] = lVar[val0[i]]
                    elif val0[i].isdigit():
                        val0[i] = int(val0[i])
                    else:
                        if val0[i].startswith('"') and val0[i].endswith('"') or val0[i].startswith("'") and val0[i].endswith("'"):
                            val0[i] = val0[i][1:-1]
                        else:
                            if val0[i] == "True" or val0[i] == "False" or val0[i] == "true" or val0[i] == "false":
                                val0[i] = val0[i].upper()
                            elif val0[i] == "NULL" or val0[i] == "NILL" or val0[i] == "null" or val0[i] == "nill" or val0[i] == "None" or val0[i] == "none":
                                val0[i] = None
                            else:
                                displayError("L-E3 : Undefined object", line)
                lVar[varName] = val0
                lAdr[varName] = id(lVar[varName])
            elif "." in varValue and varValue not in floatNotCase:
                pointCount = 0
                for i in range(0, len(varValue)):
                    if varValue[i] == ".":
                        pointCount += 1
                    else:
                        pass
                if pointCount == 1:
                    try:
                        lVar[varName] = float(varValue)
                        lAdr[varName] = id(lVar[varName])
                    except:
                        displayError("L-E3 : Undefined object", line)
                else:
                    displayError("L-E3 : Undefined object", line)
    else:
        displayError("L-E3 : Undefined object", line)

def varArray(param, line):
    varDec, varArrayValue = param.split("=", 1)
    varName = varDec.strip()
    varArrayValue = varArrayValue.strip()
    varArrayValueArray = [i.strip() for i in varArrayValue.split(",")]
    lVar[varName] = varArrayValueArray
    for i in range(0, len(varArrayValueArray)):
        if varArrayValueArray[i].isdigit():
            lVar[varName][i] = int(varArrayValueArray[i])
        elif varArrayValueArray[i] in lAct:
            lVar[varName][i] = lAct[varArrayValueArray[i]]
        elif varArrayValueArray[i] in lVar:
            lVar[varName][i] = lVar[varArrayValueArray[i]]
        else:
            if varArrayValueArray[i].startswith('"') and varArrayValueArray[i].endswith('"') or varArrayValueArray[i].startswith("'") and varArrayValueArray[i].endswith("'"):
                lVar[varName][i] = varArrayValueArray[i][1:-1]
            elif varArrayValueArray[i] == "True" or varArrayValueArray[i] == "False" or varArrayValueArray[i] == "true" or varArrayValueArray[i] == "false":
                lVar[varName][i] = varArrayValueArray[i].upper()
            elif varArrayValueArray[i] == "NULL" or varArrayValueArray[i] == "NILL" or varArrayValueArray[i] == "null" or varArrayValueArray[i] == "nill" or varArrayValueArray[i] == "None" or varArrayValueArray[i] == "none":
                lVar[varName][i] = None
            elif "." in varArrayValueArray[i] and varArrayValueArray[i] not in floatNotCase:
                pointCount = 0
                for j in range(0, len(varArrayValueArray[i])):
                    if varArrayValueArray[i][j] == ".":
                        pointCount += 1
                    else:
                        pass
                if pointCount == 1:
                    try:
                        lVar[varName][i] = float(varArrayValueArray[i])
                    except:
                        displayError("L-E3 : Undefined object", line)
                else:
                    displayError("L-E3 : Undefined object", line)
            else:
                displayError("L-E3 : Undefined object", line)
    lAdr[varName] = id(lVar[varName])

def pointer(param, line):
    if param in lVar:
        displayWarning("L-W2 : Using pointer to update existing variable is deprecated, use pointerupd next time onwards", line)
        lVar[param] = id(lVar[param])
    else:
        if param in lAct:
            lVar[param] = id(lAct[param])
        elif param.isdigit():
            lVar[param] = id(int(param, line))
        elif param.startswith('"') and param.endswith('"') or param.startswith("'") and param.endswith("'"):
            lVar[param] = id(param[1:-1])
        elif param == "True" or param == "False" or param == "true" or param == "false":
            lVar[param] = id(param.upper())
        elif param == "NULL" or param == "NILL" or param == "null" or param == "nill" or param == "None" or param == "none":
            lVar[param] = id(None)
        else:
            displayError("L-E3 : Undefined object", line)

def pointerArray(param, line):
    pointerDec, pointerArrayValue = param.split("=", 1)
    pointerName = pointerDec.strip()
    pointerArrayValue = pointerArrayValue.strip()
    pointerArrayValueArray = [i.strip() for i in pointerArrayValue.split(",")]
    lVar[pointerName] = [id(i) for i in pointerArrayValueArray]
    lAdr[pointerName] = id(lVar[pointerName])

def pointerUpdate(param, line):
    pointerName, pointerValue = param.split(" ")
    pointerName = pointerName.strip()
    pointerValue = pointerValue.strip()
    if pointerValue in lVar:
        lVar[pointerName] = id(lVar[pointerValue])
        lAdr[pointerName] = id(lVar[pointerName])
    else:
        displayError("L-E3 : Undefined object", line)

def workerFuncDef(param, line):
    try:
        varName, equal ,workerExpression = param.split(" ")
        equal = None
    except:
        displayError("L-E6 : Invalid worker function definition", line)
        return
    varName = varName.strip()
    workerExpression = workerExpression.strip()
    if workerExpression.startswith("display->"):
        function, expression = workerExpression.split("->")
        workerExpression = display(expression, line)
        workerStore[varName] = lambda : workerExpression
    elif workerExpression.startswith("var->"):
        function, expression = workerExpression.split("->")
        workerExpression = var(expression)
        workerStore[varName] = lambda : workerExpression
    elif workerExpression.startswith("varUpd->"):
        function, expression = workerExpression.split("->")
        workerExpression = varUpd(expression)
        workerStore[varName] = lambda : workerExpression
    elif workerExpression.startswith("var[]->"):
        function, expression = workerExpression.split("->")
        workerExpression = varArray(expression)
        workerStore[varName] = lambda: workerExpression
    elif workerExpression.startswith("pointer->"):
        function, expression = workerExpression.split("->")
        workerExpression = pointer(expression)
        workerStore[varName] = lambda : workerExpression
    else:
        displayError("L-E6 : Invalid operation in worker function definition", line)

def workerFuncCall(param, line):
    if param in workerStore:
        workerStore[param]()
    else:
        displayError("L-E10 : worker not defined", line)

def forLoop(param, line):
    iteratorVar, forCommand, times, action = param.split(" ", 3)
    iteratorVar = iteratorVar.strip()
    forCommand = forCommand.strip()
    times = times.strip()
    action = action.strip()
    if forCommand == "from":
        start, end = times.split("~")
        start = int(start)
        end = int(end)
        for i in range(start, end + 1):
            if action.startswith("display->"):
                display(action[9:], line)
            elif action.startswith("var->"):
                var(action[5:])
            elif action.startswith("varUpd->"):
                varUpd(action[8:])
            elif action.startswith("var[]->"):
                varArray(action[7:])
            elif action.startswith("pointer->"):
                pointer(action[9:])
            elif action.startswith("pointer[]->"):
                pointerArray(action[10:])
            elif action.startswith("pointerupd->"):
                pointerUpdate(action[12:])
            elif action.startswith("input->"):
                inputTake(action[7:])
            elif action.startswith("type->"):
                typeView(action[6:])
            elif action.startswith("typecast->"):
                typeCast(action[10:])
            elif action.startswith("bin->"):
                binView(action[5:])
            elif action.startswith("hex->"):
                hexView(action[5:])
            elif action.startswith("oct->"):
                octView(action[5:])
            elif action.startswith("varops->"):
                varOps(action[8:])
            else:
                displayError("L-E6 : Invalid operation in for loop", line)
    elif forCommand == "till":
        times = int(times)
        for i in range(0, times + 1):
            if action.startswith("display->"):
                display(action[9:], line)
            elif action.startswith("var->"):
                var(action[5:])
            elif action.startswith("varUpd->"):
                varUpd(action[8:])
            elif action.startswith("var[]->"):
                varArray(action[7:])
            elif action.startswith("pointer->"):
                pointer(action[9:])
            elif action.startswith("pointer[]->"):
                pointerArray(action[10:])
            elif action.startswith("pointerupd->"):
                pointerUpdate(action[12:])
            elif action.startswith("input->"):
                inputTake(action[7:])
            elif action.startswith("type->"):
                typeView(action[6:])
            elif action.startswith("typecast->"):
                typeCast(action[10:])
            elif action.startswith("bin->"):
                binView(action[5:])
            elif action.startswith("hex->"):
                hexView(action[5:])
            elif action.startswith("oct->"):
                octView(action[5:])
            elif action.startswith("varops->"):
                varOps(action[8:])
            else:
                displayError("L-E6 : Invalid operation in for loop", line)
    elif forCommand == "noDuckTill":
        times = int(times)
        for i in range(1, times + 1):
            if action.startswith("display->"):
                display(action[9:], line)
            elif action.startswith("var->"):
                var(action[5:])
            elif action.startswith("varUpd->"):
                varUpd(action[8:])
            elif action.startswith("var[]->"):
                varArray(action[7:])
            elif action.startswith("pointer->"):
                pointer(action[9:])
            elif action.startswith("pointer[]->"):
                pointerArray(action[10:])
            elif action.startswith("pointerupd->"):
                pointerUpdate(action[12:])
            elif action.startswith("input->"):
                inputTake(action[7:])
            elif action.startswith("type->"):
                typeView(action[6:])
            elif action.startswith("typecast->"):
                typeCast(action[10:])
            elif action.startswith("bin->"):
                binView(action[5:])
            elif action.startswith("hex->"):
                hexView(action[5:])
            elif action.startswith("oct->"):
                octView(action[5:])
            elif action.startswith("varops->"):
                varOps(action[8:])
            else:
                displayError("L-E6 : Invalid operation in for loop", line)
    else:
        displayError("L-E7 : Invalid for loop command", line)

def act(param, line):
    varName, varActValue = param.split("=")
    varName = varName.strip()
    varActValue = varActValue.strip()
    if varName in lVar:
        if varActValue.isdigit():
            lAct[varName] = int(varActValue)
        elif varActValue in lAct:
            lAct[varName] = lAct[varActValue]
        elif varActValue in lVar:
            lAct[varName] = lVar[varActValue]
        elif varActValue.startswith('"') and varActValue.endswith('"') or varActValue.startswith("'") and varActValue.endswith("'"):
            lAct[varName] = varActValue[1:-1]
        elif varActValue == "True" or varActValue == "False" or varActValue == "true" or varActValue == "false":
            lAct[varName] = varActValue.upper()
        elif varActValue == "NULL" or varActValue == "NILL" or varActValue == "null" or varActValue == "nill" or varActValue == "None" or varActValue == "none":
            lAct[varName] = None
        elif "," in varActValue:
            value = varActValue.strip()
            val0 = value.split(",")
            val0 = [i.strip() for i in val0]
            for i in range(0, len(val0)):
                if val0[i] in lAct:
                    val0[i] = lAct[val0[i]]
                elif val0[i] in lVar:
                    val0[i] = lVar[val0[i]]
                elif val0[i].isdigit():
                    val0[i] = int(val0[i])
                else:
                    if val0[i].startswith('"') and val0[i].endswith('"') or val0[i].startswith("'") and val0[i].endswith("'"):
                        val0[i] = val0[i][1:-1]
                    else:
                        if val0[i] == "True" or val0[i] == "False" or val0[i] == "true" or val0[i] == "false":
                            val0[i] = val0[i].upper()
                        elif val0[i] == "NULL" or val0[i] == "NILL" or val0[i] == "null" or val0[i] == "nill" or val0[i] == "None" or val0[i] == "none":
                            val0[i] = None
                        else:
                            displayError("L-E3 : Undefined object", line)
            lAct[varName] = val0
        elif "." in varActValue and varActValue not in floatNotCase:
            pointCount = 0
            for i in range(0, len(varActValue)):
                if varActValue[i] == ".":
                    pointCount += 1
                else:
                    pass
            if pointCount == 1:
                try:
                    lAct[varName] = float(varActValue)
                except:
                    displayError("L-E3 : Undefined object", line)
            else:
                displayError("L-E3 : Undefined object", line)
        else:
            displayError("L-E3 : Undefined object", line)
    else:
        displayError("L-E3 : Undefined object", line)

def killAct(param, line):
    if param in lAct:
        del lAct[param]
    else:
        displayError("L-E3 : Undefined object", line)

def kill(param, line):
    if param in lVar:
        del lVar[param]
        del lAdr[param]
        del lAct[param]
    else:
        displayError("L-E3 : Undefined object", line)

def killActAll():
    lAct.clear()

def killAll():
    lVar.clear()
    lAdr.clear()
    lAct.clear()

def plotGraph(param, line):
    if param in lAct:
        if type(lAct[param]) == list:
            plt.plot(lAct[param])
            plt.show()
        elif type(lVar[param]) == list:
            plt.plot(lVar[param])
            plt.show()
        else:
            displayError("L-E8 : Invalid data type", line)
    else:
        if type(param) == list:
            plt.plot(param)
            plt.show()
        elif param.isdigit():
            plt.plot(int(param))
            plt.show()
        elif "," in param:
            value = param.strip()
            val0 = value.split(",")
            val0 = [i.strip() for i in val0]
            for i in range(0, len(val0)):
                if val0[i] in lAct:
                    val0[i] = lAct[val0[i]]
                elif val0[i] in lVar:
                    val0[i] = lVar[val0[i]]
                elif val0[i].isdigit():
                    val0[i] = int(val0[i])
                else:
                    if val0[i].startswith('"') and val0[i].endswith('"') or val0[i].startswith("'") and val0[i].endswith("'"):
                        val0[i] = val0[i][1:-1]
                    else:
                        if val0[i] == "True" or val0[i] == "False" or val0[i] == "true" or val0[i] == "false":
                            val0[i] = val0[i].upper()
                        elif val0[i] == "NULL" or val0[i] == "NILL" or val0[i] == "null" or val0[i] == "nill" or val0[i] == "None" or val0[i] == "none":
                            val0[i] = None
                        else:
                            displayError("L-E3 : Undefined object", line)
            plt.plot(val0)
            plt.show()
        elif "." in param and param not in floatNotCase:
            pointCount = 0
            for i in range(0, len(param, line)):
                if param[i] == ".":
                    pointCount += 1
                else:
                    pass
            if pointCount == 1:
                try:
                    plt.plot(float(param, line))
                    plt.show()
                except:
                    displayError("L-E3 : Undefined object", line)
            else:
                displayError("L-E3 : Undefined object", line)
        else:
            displayError("L-E3 : Undefined object", line)

def fileHandling(param, line):
    param = param.split(" ")
    if not os.path.exists(param[0]):
        displayError("L-E14 : File not found", line)
    else:
        if param[1] == "read":
            file = open(param[0], "r")
            print(file.read())
            file.close()
        elif param[1] == "write":
            file = open(param[0], "w")
            file.write(param[2])
            file.close()
        elif param[1] == "append":
            file = open(param[0], "a")
            file.write(param[2])
            file.close()
        else:
            displayError("L-E15 : Invalid file operation", line)

def memClear():
    lVar.clear()
    lAdr.clear()
    lAct.clear()
    workerStore.clear()

def ping(url, line):
    try:
        if requests.get(url):
            print(f"Pinging {url}..!")
            print(f"Status : {requests.get(url).status_code}")
        else:
            displayError("L-E12 : Invalid URL", line)
    except:
        displayError("L-E12 : Invalid URL", line)