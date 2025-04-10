from liblinea import Linea
from liblinea import Error
import importlib.util
import os

def main(script):
    script = open(script, "r")
    scriptLines = script.readlines()
    for line in scriptLines:
        line = line.strip()
        if line.startswith("#") or not line:
            continue
        if line.startswith("var "):
            varName, value = line[4:].split("=", 1)
            varName = varName.strip()
            value = value.strip()
            Linea.varDeclare(varName, value, globals())
        elif line.startswith("+") or line.startswith("-") or line.startswith("*") or line.startswith("/") or line.startswith("%") or line.startswith("^"):
            Linea.varOps(line)
        elif line.startswith("varUpd "):
            varName, value = line[7:].split("=", 1)
            varName = varName.strip()
            value = value.strip()
            Linea.varUpdate(varName, value)
        elif line.startswith("display "):
            try:
                param = line[8:]
                result = Linea.breakPhraseToWords(param, globals())
                Linea.display(result)
            except Exception as e:
                Linea.displayError(f"Error: {str(e)}")
        elif line.startswith("act "):
            varName, value = line[4:].split("=", 1)
            varName = varName.strip()
            value = value.strip()
            Linea.varAct(varName, value)
        elif line.startswith("killAct "):
            toKill = line[8:]
            toKill = toKill.strip()
            Linea.varActKill(toKill)
        elif line == ("killAllAct"):
            Linea.varActKillAll()
        elif line == "@var":
            Linea.displayVars()
        elif line.startswith("@act"):
            Linea.displayActVars()
        elif line.startswith("@varDType"):
            Linea.displayVarDType()
        elif line.startswith("@actDType"):
            Linea.displayActDType()
        elif line in ["", " ", "  ", "   ", "    ", "\n", "\t"]:
            pass
        elif line.startswith("for "):
            Linea.forLoop(line[4:])
        elif line.startswith("weblet "):
            Linea.displayWarning("Usage of weblet with weblet keyword is deprecated")
            Linea.displayWarning("use liblinea.weblet instead and invoke it by typing: ")
            Linea.displayWarning("`liblinea.weblet.Weblet.weblet(param)`")
        elif line == "getMemory" or line == "getMem":
            print(Linea.getMemory())
        elif line == "getMemory available" or line == "getMem available":
            print(Linea.getMemory("Available"))
        elif line == "getMemory used" or line == "getMem used":
            print(Linea.getMemory("Used"))
        elif line == "getMemory all" or line == "getMem all":
            print(Linea.getMemory("All"))
        elif line == "getMemory usage":
            print(Linea.getMemory("MemoryUsage"))
        elif line == "getMemory free" or line == "getMem free":
            print(Linea.getMemory("Free"))
        elif line == "getMemory total" or line == "getMem total":
            print(Linea.getMemory("Total"))
        elif line.startswith("typeCast "):
            Linea.typeCast(line[9:])
        elif line.startswith("adr "):
            Linea.address(line[4:])
        elif line.startswith("use "):
            try:
                module_path = line[4:]
                spec = importlib.util.spec_from_file_location(module_path, f"{module_path}.py")
                module = importlib.util.module_from_spec(spec)
                sys.modules[module_path] = module
                spec.loader.exec_module(module)
                
                # Store the imported module in a global dictionary for later use
                globals()[module_path] = module
            except Exception as e:
                err = Error("L-E10", f"Error importing module '{module_path}': {str(e)}")
                err.displayError(err)
        elif line.startswith("exit"):
            Linea.exit()
        else:
            try:
                parts = line.split(".")
                module_name = ".".join(parts[:-2])  # e.g., "math.liblinea"
                class_name = parts[-2]  # e.g., "Basic"
                function_call = parts[-1]  # e.g., "sqrt(5)"
                func_name, args = function_call.split("(", 1)
                args = args.rstrip(")")# Get the module, class, and function
                module = globals().get(module_name)
                if not module:
                    raise ImportError(f"Module '{module_name}' not imported.")
                cls = getattr(module, class_name)
                func = getattr(cls(), func_name)
                
                # Evaluate the arguments and call the function
                func(*eval(f"[{args}]"))
            except:
                err = Error("L-E9", f"Unknown command: {line}")
                err.displayError(err)
    script.close()
    Linea.exit()

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python script.py <script_file>")
        sys.exit(1)
    script_file = sys.argv[1]
    main(script_file)