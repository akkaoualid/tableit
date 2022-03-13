# tableit
CLI app for formatting entries in a table

## Usage
```sh
$  tableit --columns="c1,c2,c3,c4" --rows="r1,r2,r3,r4|r5,r6,r7,r8"
# or
$  tableit -c "c1,c2,c3,c4" -r "r1,r2,r3,r4|r5,r6,r7,r8"


╔════╦════╦════╦════╗
║ c1 ║ c2 ║ c3 ║ c4 ║
╟════╬════╬════╬════╣
║ r1 ║ r2 ║ r3 ║ r4 ║
║ r5 ║ r6 ║ r7 ║ r8 ║
╚════╩════╩════╩════╝
```

### Styles
You can get list of available table styles by default by supplying the '--styles' or '-S' command
```sh
$ tableit --styles
# or
$ tableit -S

availables styles:
0: rst
1: single
2: light
3: fancy1
```
### Using a style
You can use the style you want by supplying the command '--style=<STYLE-NAME>' or '-s <STYLE-NAME>'
```sh
$  tableit --columns="c1,c2,c3,c4" --rows="r1,r2,r3,r4|r5,r6,r7,r8" --style="single"
# or
$  tableit --columns="c1,c2,c3,c4" --rows="r1,r2,r3,r4|r5,r6,r7,r8" -S "single"

 \---- \---- \---- \----
  c1   c2   c3   c4
 \---- \---- \---- \----
  r1   r2   r3   r4
  r5   r6   r7   r8
```
