# The tyler Language

## The world's most useless programming language™️!

## Language Definition

The tyler language (which uses extensions `.tyler` or `.ty`) is something I made because I thought it would be "fun". There are no practical applications for this language, and trying to use this language for a practical application is against the rules.

### Data Types

tyler has 3 data types, because tbh you don't need anymore.

-   Values

    This is any single value, like text or a number

-   Lists

    A list can be made up of any of the three data types

-   Execs

    A reusable bit a code, that, once defined, can be executed like any of the five core commands

### Commands

tyler has five core commands

#### t

Tells something, in other words, this is the print statement, and it works for any of the three data types  
`t $value|$list|$exec`

-   value

    Prints the value

-   list

    Prints each item in the list (essentially just calls `t` on each)

-   exec

    Prints out the code stored for this exec

#### y

Yells something, essentially the same as `t`, but it'll be in all-caps  
`y $value|$list|$exec`

#### l

Creates a list with the given name and items (space separated)  
`l $name $item1 $item2 ... $itemN`  
Items can any of the three data types

#### e

Create an exec  
`e $name $code`  
`$code` can be any of the five commands, or another exec:

-   ```
    e greeting t hello
    greeting
    ```
    would print "hello"
-   ```
    e greeting t hello
    e hello greeting
    hello
    ```
    would print "hello"
-   ```
    e initList l stuff test 1 foo
    initList
    ```
    would initialize the list "stuff" with items test, 1, and foo

#### r

This command has two different functions, depending on syntax

1. Repeat a line
   When a line with just `r` is placed below another line of the code, `r` will repeat that line's execution:

    ```
    t hello
    r
    ```

    would print out:  
     hello  
     hello

2. "Repeat" the contents of a list  
   This will iterate over all the items in a list and evaluate them based upon their type
    - value  
      Prints the value (effectively `t $value`)
    - list  
      Repeats the contents of the list (effectively `r $list`)
    - exec  
      Executes the exec
