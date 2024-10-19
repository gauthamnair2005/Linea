"""
    The Linea Programming Language
    Gautham Nair
    2024
"""
    
try:
    import matplotlib.pyplot as plt
    import os
    import sys
except ImportError as e:
    print(f"\033[91mError L-E1: {str(e)}\033[0m")
    sys.exit(1)

lVar = {}
lAdr = {}
lAct = {}
workerStore = {}
floatNotCase = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "'", '"', ",", "/", "\\", "<", ">", ";", ":", "[", "]", "{", "}", "-", "_", "+", "=", "(", ")", "!", "@", "#", "$", "%", "^", "&", "*", "~", "`", "|"]

ver = "0.2 'Bettafish' Beta 5"
dev = "Gautham Nair"

def help():
    print("The Linea Programming Language " + ver + "\n" + dev)
    print("Linea Help")
    print("Usage : linea <filename>")
    print("Printing a text : display <string/variable>")
    print("Displaying declared variables : display @var")
    print("Displaying declared actor variables : display @act")
    print("Displaying variable addresses : display @adr")
    print("Clearing the screen : clrscr")
    print("Displaying data types of declared variables : display @dataType")
    print("Displaying data type of a variable : type <variable>")
    print("Type casting a variable : typecast <datatype> <variable>")
    print("Declaring a variable : var <variable> = <value>")
    print("Updating a variable : varUpd <variable> = <value>")
    print("Declaring an array : var[] <variable> = <value1>, <value2>, <value3>, ...")
    print("Declaring a pointer : pointer <variable> = <value>")
    print("Declaring an array of pointers : pointer[] <variable> = <value1>, <value2>, <value3>, ...")
    print("Updating a pointer : pointerupd <variable> <value>")
    print("Taking input : input <variable> <query>")
    print("Binary view : bin <value>")
    print("Hexadecimal view : hex <value>")
    print("Octal view : oct <value>")
    print("Variable operations : +, -, *, /, %, **, ^")
    print("For loop : for <iterator> from <start>~<end> <action>")
    print("For loop (0 to specified) : for <iterator> till <times> <action>")
    print("For loop (1 to specified) : for <iterator> noDuckTill <times> <action>")
    print("Actors : act <variable> = <value>")
    print("Killing actors : killAct <variable>")
    print("Killing variables : kill <variable>")
    print("Killing all variables : killAll")
    print("Killing all actors : killActAll")
    print("Plotting a graph : plot <variable>")
    print("Worker function definition : worker <variable> = <expression>")
    print("Worker function call : workerCall <variable>")

def vers():
    print("Linea Programming Language " + ver + "\n" + dev)

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

def Linea(fileName):
    try:
        lineCount = 0
        if not os.path.exists(fileName):
            print("\033[91mFile not found\033[0m")
            sys.exit(1)
        else:
            with open(fileName, "r") as f:
                for line in f:
                    lineCount += 1
                    line = line.strip()
                    if line.startswith("#"):
                        pass
                    elif line.startswith("/*") and line.endswith("*/"):
                        pass
                    elif line == "ver":
                        vers()
                    elif line.startswith("display "):
                        display(line[8:], lineCount)
                    elif line == "display @var":
                        displayVar()
                    elif line == "display @act":
                        displayAct()
                    elif line == "display @adr":
                        displayAdr()
                    elif line == "clrscr" or line == "clear screen" or line == "cls" or line == "clrScr()" or line == "clearScreen()" or line == "clearScreen":
                        clrScr()
                    elif line == "display @dataType":
                        displayDataType()
                    elif line.startswith("type "):
                        typeView(line[5:], lineCount)
                    elif line.startswith("typecast "):
                        typeCast(line[9:], lineCount)
                    elif line.startswith("var "):
                        var(line[4:], lineCount)
                    elif line.startswith("varUpd "):
                        varUpd(line[7:], lineCount)
                    elif line.startswith("var[] "):
                        varArray(line[6:], lineCount)
                    elif line.startswith("pointer "):
                        pointer(line[8:], lineCount)
                    elif line.startswith("pointer[] "):
                        pointerArray(line[10:], lineCount)
                    elif line.startswith("pointerupd "):
                        pointerUpdate(line[12:], lineCount)
                    elif line.startswith("input "):
                        inputTake(line[6:], lineCount)
                    elif line.startswith("bin "):
                        binView(line[4:], lineCount)
                    elif line.startswith("hex "):
                        hexView(line[4:], lineCount)
                    elif line.startswith("oct "):
                        octView(line[4:], lineCount)
                    elif line.startswith("+ ") or line.startswith("- ") or line.startswith("* ") or line.startswith("/ ") or line.startswith("% ") or line.startswith("** ") or line.startswith("^ "):
                        varOps(line, lineCount)
                    elif line.startswith("for "):
                        forLoop(line[4:], lineCount)
                    elif line.startswith("act "):
                        act(line[4:], lineCount)
                    elif line.startswith("killAct "):
                        killAct(line[8:], lineCount)
                    elif line.startswith("kill "):
                        kill(line[5:], lineCount)
                    elif line.startswith("worker "):
                        workerFuncDef(line[7:], lineCount)
                    elif line.startswith("workerCall "):
                        workerFuncCall(line[11:], lineCount)
                    elif line == "killAll":
                        killAll()
                    elif line == "killActAll":
                        killActAll()
                    elif line.startswith("plot "):
                        plotGraph(line[5:], lineCount)
                    else:
                        try:
                            print(eval(line))
                        except:
                            displayError("L-E9 : Invalid keyword", lineCount)
    except:
        displayError("L-E11 : Undefined error", lineCount)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Linea Programming Language " + ver + "\n" + dev)
        print("\033[91mNo input file specified\033[0m")
        print("linea <filename>")
        sys.exit(1)
    fileName = sys.argv[1]
    if fileName == "--v" or fileName == "-version":
        vers()
        sys.exit(0)
    elif fileName == "--h" or fileName == "-help":
        help()
        sys.exit(0)
    elif fileName == "--d" or fileName == "-developer":
        print(dev)
        sys.exit(0)
    elif fileName == "--l" or fileName == "-license":
        print("GNU GPL v3")
        sys.exit(0)
    elif fileName == "--c" or fileName == "-credits":
        print("Linea Programming Language " + ver + "\n" + dev)
        sys.exit(0)
    elif fileName == "--a" or fileName == "-about":
        print("Linea Programming Language " + ver + "\n" + dev)
        sys.exit(0)
    elif fileName == "--i" or fileName == "-info":
        print("Linea Programming Language " + ver + "\n" + dev)
        sys.exit(0)
    else:
        if fileName.startswith("-"):
            print("Linea Programming Language " + ver + "\n" + dev)
            print("\033[91mInvalid argument\033[0m")
        else:
            Linea(fileName)
            sys.exit(0)