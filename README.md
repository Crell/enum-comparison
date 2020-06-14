
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


### Python

Python builds its enum support on top of classes.  An "enum class" is simply a class that extends the `enum.Enum` parent, which has a lot of methods pre-implemented to provide Enum-ish behavior.  All properties of the class are enum members:

```python
import enum

class Suit(enum.Enum):
    HEARTS = enum.auto()
    DIAMONDS = enum.auto()
    CLUBS = 'C'
    SPADES = "S"
```

Enum members can be any int or string primitive, or can be auto-generated.  The auto-generation logic can also be overridden by defining a `_generate_next_value_()` method in the class.  When an enum value is cast to a string, it always shows as `Card.CLUBS` or similar, but can be overridden by implementing the `__str__` method.

Enum member names must be unique, but values need not be.  If two members have the same value then the syntactically first one wins, and all others are alises to it.  The aliases will be skipped when iterating an enum or casting it to a list.  If needed, you can get the original list with `Card.__members__.items()`.

As a class, an enum can also have methods.  However, the methods have no native way to vary depending on which enum value they're on.  You can check the value within the method, though:

```python
class Suit(enum.Enum):
    HEARTS = enum.auto()
    DIAMONDS = enum.auto()
    CLUBS = 'C'
    SPADES = "S"

    def color(self):
        if self in [self.CLUBS, self.SPADES]:
            return "Black"
        else:
            return "Red"
```

Because Python lacks any meaningful type declarations on variables, parameters, or return values, there's no way to restrict a value to an enum list.  Enum classes also cannot be extended.

The `Enum` class also has an alternate function-style syntax for simple cases:

```python
Suit = Enum('Suit', 'HEARTS DIAMONDS CLUBS SPADES')
```

Further reading: https://docs.python.org/3/library/enum.html

### Typescript

Typescript supports primitive enumerations, including both constant and runtime-defined values.  Depending on the details they may or may not get compiled away to literal constants in code.  It has its own dedicated keyword.

```typescript
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
```

is equivalent to

```typescript
enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades =3,
}
```

Enums can also have string values if specified explicitly.  Values can be set based on some other value, even function definitions:

```typescript
enum FileAccess {
    // constant members
    None,
    Read    = 1 << 1,
    Write   = 1 << 2,
    ReadWrite  = Read | Write,
    // computed members
    UserSetting = userDefaultValue()
}
```

Normally enums exist at runtime, but a fully-constant enum can also be flagged to compile-away to raw constants in the source code:

```typescript
const enum ShouldWe {
    No,
    Yes,
}
```

Enum types can be used as type declarations:

```typescript
function pickCard(desiredSuit: Suit): Card { }
```

Further reading: https://www.typescriptlang.org/docs/handbook/enums.html

### Rust


### Haskell

Strictly speaking Haskell doesn't have enums, but the way its type system works gives you something close enough that I'm going to include it.  In Haskell, you define a new data type with the `data` keyword, which can be defined in terms of other data types and type constructors.

It's really hard to explain without going into the whole type system, so I'll stick to some examples:

```haskell
data Suit = Hearts | Diamonds | Clubs | Spades
```
The type "Suit" has only four values, one for each suit.  They are not backed by a primitive value but literally are those values only.  Haskell doesn't have methods as we'd understand them in the OOP world, so methods cannot be attached to them.  The can, however, be used in pattern matching:

```haskell
data Color = Red | Black

suitColor :: Suit -> Color
suitColor Hearts | Diamonds = Red
suitColor Clubs | Spades = Black
```

Because type values are technically not values but "type constructors" they can be parameterized by other values.  For instance, the infamous Maybe Monad is defined as:

```haskell
data Maybe a = Just a | Nothing
```

That is, a "Maybe" can be either the literal `Nothing` or a `Just` combined with some other value, which can then be extracted later using pattern matching.

```haskell
stuff :: Maybe a -> Int
stuff Nothing = 0
stuff Just a = a
```

Further reading: https://wiki.haskell.org/Type

### F#


### C#


### Swift

