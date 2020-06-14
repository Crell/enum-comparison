
https://twitter.com/Tojiro/status/823286025535393792


### C

In C, enumerations are really just a wrapper for named integer constants.  They are defined with the keyword `enum`, although to be fully useful they need to also be defined as a `typedef`.  For example:

```c
#include<stdio.h>

typedef enum { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday } Day;

void printer(Day d) {
  printf("The day is: %d\n", d);
}

int main() {
  Day d = Tuesday;

  printer(d);
  printer(4);
   return 0;
}
```

Even though `printer()` takes a `Day` parameter, passing an integer literal works fine.  If no integer value for an enum element is specified, the compiler assigns one automatically starting from 0.  So `Monday` is 0, `Tuesday` is 1, etc.  You can specify an equivalent integer for an enum value, including making multiple values refer to the same integer:

```c
typedef enum {
  Working,
  Failed = 5,
  Busted = 5;
} Status;
```

### Java

In Java, enums are, unsurprisingly, a shorthand for classes with class constants.  They can be defined standalone or within a class, since Java supports inner classes.  As a result, enums can support arbitrary methods.  The specific values can map to internal integer values, or they can be auto-assigned by the compiler.

The simple case looks like this:

```java

```

Enums do not support constructors.  (Or rather, the constructor is private so you cannot pass parameters to it.)