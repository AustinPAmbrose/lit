# Lit - Control your smart lights from the command line!

## Installation
To install **lit**, run one of the following commands

Mac OS:
```bash
sudo rm /usr/local/bin/lit; sudo curl https://raw.githubusercontent.com/AustinPAmbrose/lit/main/bin/lit -o /usr/local/bin/lit; sudo chmod a+x /usr/local/bin/lit
```

Windows 10:
```cmd
DEL %HOMEPATH%\lit.exe && curl https://raw.githubusercontent.com/AustinPAmbrose/lit/main/bin/lit -o %HOMEPATH%\lit.exe
```

NOTE: For Windows 10 users, lit.exe installs into your home directory (C:\Users\username). If you don't modify you're path environment variable, you'll only be able to run lit.exe when you're in your home folder.

## Getting Started
To access the help menu for lit, run
```bash
lit --help
```

To set the brightness of all lights to 100%, run
```bash
lit 100
```

To get the current state of your lights, run
```bash
lit -d
```

To turn off all lights, run
```bash
lit 0
```

To turn on the lights in the living room and poker room use the -k and -p flags
```bash
lit -k -p 100

# Or, equivilently
lit -kp 100
```
