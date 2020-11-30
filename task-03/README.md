# HackerRank Tasks
## Great Eye
This task can broken down into two simple conditions:-
1. Either the number the user entered exists as one of the positions of the word in the sentence. (Basically then find ascii sum of the values)
2. The number doesn't exist as a position. (Print -1)

A simple for loop through the string allowed me to extract each letter of the word and ultimately its ascii value.

## Ryuk and the Death Notes
The first thing I had to think about was how was I going to compare the required no. of pages and the pages which Ryuk had. Then, I had to extract the possible number of death notes that could be made.
In the end here's what I came up with:-
1. Store both the set of values ( required no. of pages and the pages which Ryuk had) in arrays. 
2. Run through both the arrays using a single for loop
3. Extract the integer value by dividing available pages and required pages. (basically possible number of notes that could be created with each type of page)
4. Print the highest of the values extracted (So, possible number of death notes)

## Homework Time
There are 3 parts to this problem.
1. Create the Incredible Series
2. Correspond the position given by the user to the number in the series
3. Print it in reverse.

For 1. A for loop and the usage of an array enabled me to create the series.
For 2. Again, a for loop helped me with this
For 3. For this, I converted the number into a string and set condition to remove the first value and so on if there was zero after reversal.






