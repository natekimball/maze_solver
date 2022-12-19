# Maze Solver
Over-engineered maze solving algorithms written in rust.

## Examples

### Maze input example
```
resources/21x60maze.txt:
``` 
```
XXSXXXXXX     XXXXX   XXXXXXXXXX     XXXXXXXXXXXXXXXXXXXXX X
XX XXXX   XXX XXXXX X XXX     X  XXX          XXX       XX X
   XXX  X   X       X     XXXXX XX XX XXX XXX XXX XXXXX X  X
 XXXXX X  X XXXXXXXXX X XXX     XX XX XXX XXX     XXXXX   XX
    XX X XX  X    XXX X XX  XXXXXX XX XXX XXXXXXXXXXXXXXX   
XXX XX X XXX X     XX   XX XXXX       XXX XXXXXXXXXXXXXXXXXX
       X XXX XXXXX XXXXXXX XXXXXXXXXXXXXX                  X
XXXXXXXX XXX     X XXX                XXX XXX XXXXXXXXXXXXXX
XX       XXXXXXX XXXXX XXXXXX XXXXXXXX    X X           X    
XX XXXXXXXXXXXXX       XXXXXX XXX      XXXX XXXXXXXXXXX X X 
XX X          XXXXXXXXXX      XXX XXXXX XXX           X   X 
XX X XXXXXXXX X          XXXX    XXX XX     XXXXXXXXX XXXXX 
   X       XX X XXXXXXXXX   X XX  XX  X XXXXX             X 
X XXXXXXXXXXX X X         X X XXX XXX X XXXXXXXXXXXXXXXXX X 
X XXXXXXXXXXX X X XXXXXXXXX   XXX    XX X                 X 
X             X XXXXXXXXXXX XXXXXXXX XX X X XXXXXXXX XXXXXX 
XX XXXXXXX XXXX             X XXXXXX XX X X  X        XXX X 
XX  XXX XX XXXXXXXXXXXXXXXXXX        XX   X XXXXXXXX XXXX X 
XXX XXX XX                    X XXXXXXX X X      X        X 
XXX X X XXXXXXXXXXXXXXXXX XXXXX XXXXXXX X XXXXXXXXXXXXX XXX 
XXX           XXXXXX       XXXXE        X                  
```
### Find all solutions to a maze
```
$ cargo run . -f resources/13x7maze.txt
```
### Find the shortest path in a maze
```
$ cargo run . -f resources/13x7maze.txt -s
```
### Find any solution to a maze
```
$ cargo run . -f resources/13x7maze.txt -a
```
### Shortest Path Output Example
```
shortest path length: 77
solution:
XXSXXXXXX     XXXXX   XXXXXXXXXX     XXXXXXXXXXXXXXXXXXXXX X
XX|XXXX|||XXX XXXXX X XXX     X  XXX          XXX       XX X
|||XXX||X|||X       X     XXXXX XX XX XXX XXX XXX XXXXX X  X
|XXXXX|X  X|XXXXXXXXX X XXX     XX XX XXX XXX     XXXXX   XX
||||XX|X XX||X    XXX X XX  XXXXXX XX XXX XXXXXXXXXXXXXXX   
XXX|XX|X XXX|X     XX   XX XXXX       XXX XXXXXXXXXXXXXXXXXX
   ||||X XXX|XXXXX XXXXXXX XXXXXXXXXXXXXX                  X
XXXXXXXX XXX|||||X XXX||||||||        XXX XXX XXXXXXXXXXXXXX
XX       XXXXXXX|XXXXX|XXXXXX|XXXXXXXX    X X           X    
XX XXXXXXXXXXXXX|||||||XXXXXX|XXX      XXXX XXXXXXXXXXX X X 
XX X          XXXXXXXXXX     |XXX XXXXX XXX           X   X 
XX X XXXXXXXX X          XXXX||||XXX XX     XXXXXXXXX XXXXX 
   X       XX X XXXXXXXXX   X XX||XX  X XXXXX             X 
X XXXXXXXXXXX X X         X X XXX|XXX X XXXXXXXXXXXXXXXXX X 
X XXXXXXXXXXX X X XXXXXXXXX   XXX||||XX X                 X 
X             X XXXXXXXXXXX XXXXXXXX|XX X X XXXXXXXX XXXXXX 
XX XXXXXXX XXXX             X XXXXXX|XX X X  X        XXX X 
XX  XXX XX XXXXXXXXXXXXXXXXXX  ||||||XX   X XXXXXXXX XXXX X 
XXX XXX XX                    X|XXXXXXX X X      X        X 
XXX X X XXXXXXXXXXXXXXXXX XXXXX|XXXXXXX X XXXXXXXXXXXXX XXX 
XXX           XXXXXX       XXXXE        X                  
```