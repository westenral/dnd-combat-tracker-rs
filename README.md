# DND Combat Tracker
Makes large combat scenarios easier by tracking initiative and health

## Usage
The program reads the starting information from a file. When run without arguments it attempts to load `inits.txt`. Alternatively, specify the path in the first command line argument: `cargo run <filepath>`.

### Correct Input File Format
The program panics if the rows of the input file are incorectly formatted. Correct format is as follows:

`<initiative> <name> <hp> <damage>`

HP and damage are optional; initiative and name are required.

An example `inits.txt`:
```
11 arthur
3 wolf1 11
13 wolf2 11 3
19 stux
```

If run with this as the input, the program should spit out something like:
```
Loading inits.txt

stux    19 init         0 hp
wolf2   13 init         8 hp
arthur  11 init         0 hp
wolf1   3 init          11 hp
```

### Modifying, Reading, and Saving the Combat State
The `dmg`, `print`, and `save` commands can be issued to modify the state of the combat. An example output is as follows:
```
Loading inits.txt

stux    19 init         0 hp
wolf2   13 init         8 hp
arthur  11 init         0 hp
wolf1   3 init          11 hp

"dmg <name> <damage>"   deal damage
"save <filepath>"       output to file
"print"                 print current state
"quit"                  exit program

dmg wolf1 6
wolf1 took 6 damage

print

stux    19 init         0 hp
wolf2   13 init         8 hp
arthur  11 init         0 hp
wolf1   3 init          5 hp

dmg wolf1 8
wolf1 took 8 damage
wolf1 has died; removing from turn order

print

stux    19 init         0 hp
wolf2   13 init         8 hp
arthur  11 init         0 hp

dmg arthur 3
arthur took 3 damage

save save_example.txt
Saved to save_example.txt

quit
```
The file `save_example.txt` will then contain:
```
19 stux
13 wolf2 11 3
11 arthur
```
Note that the program will never overwrite a pre-existing file.
