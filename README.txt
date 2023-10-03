DND Initiative Tracker
Makes keeping track of initiatives easier by automatically sorting whose turn it is

Usage
The program will read from a file specified in a command line argument
If no argument is specified, it will look for inits.txt

The rows of the file should be in the following format
<Initiative> <Name>

An example inits.txt:
11 Arthur
3 Wolf1
19 Stux

If run with this as the input, the program should spit out:
Stux	(19)
Arthur	(11)
Wolf1	(3)
