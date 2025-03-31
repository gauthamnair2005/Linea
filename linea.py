try:
    from liblinea import *
except ImportError as e:
    print(f"\033[91mError L-E1: {str(e)}\033[0m")
    sys.exit(1)


ver = "1.8 'Mishka'"
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
    print("Linea Weblet : web <html content>")
    print("Sleep : sleep <seconds>")
    print("Permutations : permutations <n, r>")
    print("Combinations : combinations <n, r>")
    print("Factorial : factorial <n>")
    print("GCD : gcd <a, b>")
    print("LCM : lcm <a, b>")
    print("HCF : hcf <a, b>")
    print("Mean : mean <value1>, <value2>, <value3>, ...")
    print("Median : median <value1>, <value2>, <value3>, ...")
    print("Mode : mode <value1>, <value2>, <value3>, ...")
    print("Variance : variance <value1>, <value2>, <value3>, ...")
    print("Standard Deviation : stdev <value1>, <value2>, <value3>, ...")
    print("Runge-Kutta : rungeKutta <x0, y0, x, h>")
    print("Euler : euler <x0, y0, x, h>")
    print("Newton-Raphson : newtonRaphson <x0, f, f1>")
    print("Bisection : bisection <a, b, f>")
    print("Secant : secant <x0, x1, f>")
    print("Trapezoidal : trapezoidal <a, b, f, n>")
    print("Simpson's : simpsons <a, b, f, n>")
    print("Getting memory usage : getMemory")

def vers():
    print("Linea Programming Language " + ver + "\n" + dev)

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
                    elif line.startswith("timeout "):
                        time.sleep(int(line[8:]))
                    elif line.startswith("web "):
                        web(line[4:], lineCount)
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
                    elif line.startswith("sum "):
                        print(sum([int(i) for i in line[4:].split(",")]))
                    elif line.startswith("max "):
                        print(max([int(i) for i in line[4:].split(",")]))
                    elif line.startswith("min "):
                        print(min([int(i) for i in line[4:].split(",")]))
                    elif line.startswith("avg "):
                        print(sum([int(i) for i in line[4:].split(",")]) / len([int(i) for i in line[4:].split(",")]))
                    elif line.startswith("len "):
                        print(len([int(i) for i in line[4:].split(",")]))
                    elif line.startswith("sort "):
                        print(sorted([int(i) for i in line[5:].split(",")]))
                    elif line.startswith("reverse "):
                        print(sorted([int(i) for i in line[8:].split(",")], reverse=True))
                    elif line.startswith("shuffle "):
                        print(random.shuffle([int(i) for i in line[8:].split(",")]))
                    elif line.startswith("random "):
                        print(random.choice([int(i) for i in line[7:].split(",")]))
                    elif line.startswith("mean "):
                        print(statistics.mean([int(i) for i in line[5:].split(",")]))
                    elif line.startswith("median "):
                        print(statistics.median([int(i) for i in line[7:].split(",")]))
                    elif line.startswith("mode "):
                        print(statistics.mode([int(i) for i in line[5:].split(",")]))
                    elif line.startswith("sqrt "):
                        print(math.sqrt(int(line[5:])))
                    elif line.startswith("pow "):
                        print(math.pow(int(line[4:].split(",")[0]), int(line[4:].split(",")[1])))
                    elif line == "pi" or line == "π":
                        print(math.pi)
                    elif line.startswith("file "):
                        fileHandling(line[5:], lineCount)
                    elif line == "memClear()":
                        memClear()
                    elif line.startswith("ping "):
                        ping(line[5:], lineCount)
                    elif line.startswith("sleep "):
                        time.sleep(int(line[6:]))
                    elif line.startswith("permutations "):
                        permutations(line[13:], lineCount)
                    elif line.startswith("combinations "):
                        combinations(line[13:], lineCount)
                    elif line.startswith("factorial "):
                        factorial(line[10:], lineCount)
                    elif line.startswith("gcd "):
                        gcd(line[4:], lineCount)
                    elif line.startswith("lcm "):
                        lcm(line[4:], lineCount)
                    elif line.startswith("hcf "):
                        hcf(line[4:], lineCount)
                    elif line.startswith("fibonacci "):
                        fibonacci(line[9:], lineCount)
                    elif line.startswith("prime "):
                        prime(line[6:], lineCount)
                    elif line.startswith("trapeziodal "):
                        trapeziodal(line[12:], lineCount)
                    elif line.startswith("simpson "):
                        simpsons(line[8:], lineCount)
                    elif line.startswith("newtonRaphson "):
                        newtonRaphson(line[14:], lineCount)
                    elif line.startswith("bisection "):
                        bisection(line[10:], lineCount)
                    elif line.startswith("secant "):
                        secant(line[7:], lineCount)
                    elif line.startswith("rungeKutta "):
                        rungeKutta(line[11:], lineCount)
                    elif line == "getMemory()":
                        getMemory()
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