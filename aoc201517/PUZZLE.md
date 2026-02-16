# Day 17: No Such Thing as Too Much
The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to move it into smaller containers. You take an inventory of the capacities of the available containers.

For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there are four ways to do it:

```
15 and 10
20 and 5 (the first 5)
20 and 5 (the second 5)
15, 5, and 5
```
Filling all containers entirely, **how many different combinations of containers can exactly fit all 150 liters of eggnog?**
```
00001       5
00010       5
00011       10
00100       10
00101       15
00110       15
00111       20
01000       15
01001       20
01010       20
01011       25  X
01100       25  X
01101       30
01110       30
01111       35
10000       20
10001       25  X
10010       25  X
10011       30
10100       30
10101       35
10110       35
10111       40
11000       35
11001       40
11010       40
11011       45
11100       45
11101       50
11110       50
11111       55
```