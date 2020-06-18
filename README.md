# A survey of programming language enum support

## Introduction

As of mid-2020, there is some discussion of adding enumerations (enums) to PHP.  There are many good reasons to do so, most around enabling better data modeling and type checking, but that doesn't suggest how to do it.  Enumerations in practice refer to a very wide range of functionality depending on the language, from barely above constants to a core part of the type system.

As I am wont to do, I decided the best thing to do would be to survey the existing marketplace and see what other languages did, and what we can steal outright.  (As the saying goes, "PHP evolves by beating up other languages in dark alleys and going through their pockets for loose syntax.")  I therefore looked at 11 different languages with some kind of native enumeration support.  The survey below is intended as a reasonably fair overview and summary of the available languages.  My own thoughts and analysis are included at the end.  For some languages I have included runnable sample code in the appropriate subdirectory.  Whether or not there is sample code depends primarily on whether I had a runtime for the language already installed.

I deliberately excluded languages with no native enum support.  Languages such as Javascript, Go, or Ruby do not (as far as I can tell) have any native enumerations, although there are various hacky ways to simulate them in user space.  That is not of interest to us at this time.

If you spot any errors in the survey below, please let me know.

## Survey

### C

In C, enumerations are really just a wrapper for named integer constants.  They are defined with the keyword `enum`, usually in combination with `typedef`.  For example:

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
enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES
}
```

Enums do not support constructors.  (Or rather, the constructor is private, so you cannot pass parameters to it.)  They do, however, support interfaces.

Enum values have a number of methods on them by default to access metadata, including `Suit.valueOf("HEARTS")` (returns "HEARTS") and `Suit.valueOf("HEARTS").ordinal()` (returns 0).

The values of an enum can be iterated as a set:

```java
for (Suit s : Suit.values()){  
    System.out.println(s);  
} 
```

Because they're built on classes, enums can have methods.

```java
enum Suit {
  HEARTS,
  DIAMONDS,
  CLUBS,
  SPADES;

  public String color() {
    switch (this) {
      case SPADES:
        return "Swords of a soldier";
      case CLUBS:
        return "Weapons of war";
      case DIAMONDS:
        return "Money for this art";
      default:
        return "Shape of my heart";
    }
  }
}
```

The switch statement is not exhaustive on enums, however.

Further reading: https://www.javatpoint.com/enum-in-java

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

### Haskell

Strictly speaking Haskell doesn't have enums, but the way its type system works gives you something close enough that I'm going to include it.  In Haskell, you define a new data type with the `data` keyword, which can be defined in terms of other data types and type constructors.

It's really hard to explain without going into the whole type system, so I'll stick to some examples:

```haskell
data Suit = Hearts | Diamonds | Clubs | Spades
```

The type "Suit" has only four values, one for each suit.  They are not backed by a primitive value but literally are those values only.  Haskell doesn't have methods as we'd understand them in the OOP world, and I've not been able to wrap my brain around Haskell enough to say if you can attach methods consistently to types of an Enum.  The can, however, be used in pattern matching:

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

F#, in what seems to be a very on-brand move, has both union types *and* enums.  They are very similar but not quite the same thing.

Union types in F# look and act an awful lot like Haskell, including the requirement that the unioned types start with a capital.

```f#
type SuitUnion = Hearts | Diamonds | Clubs | Spades
```

They have no underlying primitive equivalent.  F#'s `match` directive forces you to enumerate all possible values, to help avoid errors:

```f#
let color = match x with 
    | Hearts -> Red
    | Diamonds -> Red
    | Clubs -> Black
    | Spades -> Black
```

Enums in F#, by contrast, are backed by underlying integer primitives that you specify.  Strings are not allowed.  They can be all lowercase if you want, but have to be qualified when referencing to them:

```f#
type SuitEnum = Hearts = 1 | diamonds = 2 | Clubs = 3 | Spades = 4

let color = match x with 
    | SuitEnum.Hearts -> "Red"
    | SuitEnum.diamonds -> "Red"
    | SuitEnum.Clubs -> "Black"
    | SuitEnum.Spades -> "Black"
    | _ -> "What kind of deck are you using?"
```

Enums can be cast to and from integers.  That also, oddly, allow you to define an enum value that is out of range.

```f#
// This is, amazingly, legal.
let horseshoe = enum<SuitEnum>(5)
```

For that reason, the `_` fallback match arm is required for enums, but not for unions.

Because F# doesn't have function parameter or return types, neither unions nor enums can be type defined in a function signature.

Further reading: https://fsharpforfunandprofit.com/posts/enum-types/

### C#

C# enums are explicitly just named integer constants, much like in C.  They can be defined within a class like constants, or (I think) stand-alone with a namespace.

```csharp
enum Suits 
{
    Hearts = 0,
    Diamonds,
    Clubs,
    Spades
}
```

If a value is not specified, it will be set to the highest existing value + 1.  0 is the default first value but you can set your own.  They are referenced scoped, so `Suits.Diamonds`, `Suits.Spaces`, etc.

Values can also be defined based on other enum values, bitmask style, such as `RedCards = Hearts|Diamonds`.  However, that only works if the explicit values are defined as bit flags.

Enums need to be cast to an integer explicitly in order to use as an int.

```csharp
Console.WriteLine((int)WeekDays.Monday);
```

An `Enum` class contains various static methods for manipulating enumerations further.  For instance, to get a list of the names in a given enumeration:

```csharp
foreach (string str in Enum.GetNames(typeof(WeekDays))) {
    Console.WriteLine(str);
}
```

Or this somewhat crazy way to cast an integer up to an enum member:

```csharp
WeekDays wdEnum;
Enum.TryParse<WeekDays>("1", out wdEnum);
Console.WriteLine(wdEnum);
```

Although they're not a class, you can technically add "extension methods" to enums that end up looking kind of like them.  For instance:

```csharp
public static string Color(this Suit s) {
    switch (s)
    {
        case Hearts: return "Red";
        case Diamonds: return "Red";
        case Clubs: return "Black";
        case Spades: return "Black";
    }
}

var theColor = Suit.Clubs.Color();
```

Further reading: https://www.tutorialsteacher.com/csharp/csharp-enum

### Swift

Swift's enumerations are closer to union types, but still called enumerations.  (Go figure.)  They form a full fledged type with limited legal values.  That means the type has to be capitalized, and the values not.

```swift
enum Suit {
    case hearts
    case diamonds
    case clubs
    case spaces
}
// or
enum Suit {
    case hearts, diamonds, clubs, spaces
}
```

Once defined, values can be defined of that type, and Swift's type inference capability can shorten the syntax somewhat.

```swift
var card = Suit.clubs

// since card is now bound to the type Suit, you can now do this:
card = .spades
```

You can match on an enum value with `switch`, and it must either be exhaustive or have a default:

```swift
switch card {
    case .spades:
        print("The swords of a soldier.")
    case .clubs:
        print("Weapons of war.")
    case .diamonds:
        print("Money for this art.")
    default:
        print("That's not the shape of my heart.")
}
```

Enums are not natively iterable, but they can be converted into that easily:

```swift
enum Suit: CaseIterable {
    case hearts, diamonds, clubs, spaces
}

for s in Suit.allCases {
    print(s)
}
```

Swift allows enums to have what it calls "associated values," creating what is variously called a "discriminated union" or "tagged union" depending on whom you ask.  Each value can have its own set of associated values that could be the same or different.

```swift
case Suit {
    case hearts(String)
    case diamonds(String)
    case clubs(String)
    case spades(String)
}

var threeOfDiamonds = Suit.diamond("3")
```

Each instance of an associated value enum is then not equal to another, even if they're of the same enum value.  Seemingly the only way to get those values back out, though, is with pattern matching:

```swift
switch card {
    case .spades(let value):
        print("The \(value) of Spades")
    case .clubs(let value):
        print("The \(value) of Clubs")
    case let .diamonds(value):
        print("The \(value) of Diamonds")
    case let .hearts(value):
        print("The \(value) of Hearts")
}
```

For one-off cases, you can use `if let`.

```swift
if case let .clubs(val) = card {
    print ("The \(val) of Clubs")
}
```

(Those all do the same thing, but digging into the intricacies of Swift's pattern matching is out of scope for now.)

Enums can *also* support "raw values," if specified explicitly, but they must be of the same primitive type:

```swift
enum Suit: Character {
    case hearts = "H"
    case diamonds = "D"
    case clubs = "C"
    case spades = "S"
}
```

If you list only one raw value, Swift will try to generate a raw value for the rest based on the type used.  It's also possible to initialize an enum case from a raw value, if one was defined:

```swift
let card = Suit(rawValue: "B")
```

This actually creates an "optional" of type `Suit?`, meaning it may or may not be legal and you have to explicitly check it.  (Optionals are essentially a syntactic Maybe Monad, and way off topic.)

You can even define an enumeration in terms of itself, which is just all kinds of weird.  From the documentation:

```swift
    indirect enum ArithmeticExpression {
        case number(Int)
        case addition(ArithmeticExpression, ArithmeticExpression)
        case multiplication(ArithmeticExpression, ArithmeticExpression)
    }
```

And they go further by supporting methods on enumerations of all of the above types, the body of which would meaningfully have to be a switch:

```swift
case Suit {
    case hearts(String)
    case diamonds(String)
    case clubs(String)
    case spades(String)

    func color: String {
        switch self {
            case .hearts: return "Red"
            case .diamonds: return "Red"
            case .clubs: return "Black"
            case .spades: return "Black"
        }
    }
}

print(Suit.clubs("3").color());
// Prints "Black"
```

Further reading: https://docs.swift.org/swift-book/LanguageGuide/Enumerations.html

### Rust

As Rust's main syntactic goal seems to have been "Haskell, but with lots of curly braces," the language supports enumerations with and without associated values, either positional or named.  

All of the following are legal:

```rust
// The values themselves.
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// With one or more tuple associated values.
enum Card {
    Hearts(i8),
    Diamonds(i8),
    Clubs(i8),
    Spades(i8),
}

// With one or more struct-associated values.
enum Card {
    Hearts{val: i8},
    Diamonds{val: i8},
    Clubs{val: i8},
    Spades{val: i8},
}

// With an integer (only) explicit value.
enum Suit {
    Hearts = 3,
    Diamonds = 4,
    Clubs = 5,
    Spades = 6,
}
```

Enum values can be referenced scoped from their type, `Suit::Heart`, or first imported with `use Suit::*` and then used unqualified.  Because they're a full type, they can be used in function signatures.

Enums are almost always used with either `match` or `if let`, the latter of which being a sort of inverted way to care about only a single branch of a match.  The `match` version must be exhaustive or have a default.

```rust
let msg = match card {
    Suit::Spades => "Swords of a soldier".to_string(),
    Suit::Clubs => "Weapons of war".to_string(),
    Suit::Diamonds => "Money for this art".to_string(),
    _ => "Shape of my heart".to_string(),
};

if let Diamonds(val) = card {
    println!("{} diamonds are a girl's best friend", val)
}
```

The only way to extract associated values out of the enum is with pattern matching, which in Rust is almost absurdly robust:

```rust
use Card::*;
let the_val = match Card {
    Clubs(x) | Hearts(x) | Spades(x) | Diamonds(x) => x
};
```

As it's a full type, it can also have methods.  Or in Rust-speak, "implementations," including of traits (what most languages would call an interface).  They can do pretty much everything a `struct` can.  Their body will in most cases be just a bit `match`.

```rust
impl Suit {
    fn color(&self) -> String {
        match self {
            Self::Hearts => "Red".to_string(),
            Self::Diamonds => "Red".to_string(),
            Self::Clubs => "Black".to_string(),
            Self::Spades => "Black".to_string(),
        }
    }
}
```

(The capitalized `Self` in this case is an implicit alias to `Suit`.)

Further reading: https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

### Kotlin

Kotlin also has not one but two enum-esque systems: Enums and Sealed Classes.  The difference between them is subtle and confusing.

Enums are a class that inherits from an Enum class implicitly.

```kotlin
enum class Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES
}
```

Each enum value is technically a "constant object."  By default they're bare, but can also take int or string values constructor-style.

```kotlin
enum class Suit(val abbrev: String) {
    HEARTS("H"),
    DIAMONDS("D"),
    CLUBS(C"),
    SPADES("S")
}
```

Enums in Kotlin support methods, and unlike most languages here the methods may be defined separately for each value.  Technically they're all implemented as subclasses, with the parent as an abstract base class.  An enum can even support interfaces.

```kotlin
interface Colorable {
    fun color()
}

enum class Suit(val abbrev: String) {
    HEARTS("H") {
        override fun color(): String = "Red"
    },
    DIAMONDS("D") {
        override fun color(): String = "Red"
    },
    CLUBS(C") {
         override fun color(): String = "Black"
     },
    SPADES("S") {
        override fun color(): String = "Black"
    };

    abstract fun color(): String

    fun abbreviation(): String {
        return this.abbrev
    }
}
```

Enums have a number of built-in methods and properties, which make it possible to iterate an enum or get its value.  That also makes them trivially serializable, unlike Sealed Classes.

Sealed Classes, meanwhile, are almost like normal classes except that the list of subclasses is fixed at compile time and they must appear in the same source file.

Whereas Enums are singletons, sealed classes may be singleton or instance-based.

```kotlin
sealed class Action

// This is a singleton sealed class
object Quit: Action()

// This is an instance-able sealed class
class Move(val dir: String): Action()   
```

Since they're objects/methods in their own right, they can have whatever methods you want, inherited or not.  However, they do not have the automatic methods or properties of Enums that make them serializable.

Kotlin supports a `when` syntax as an alternative to `switch` that is an expression, and can, in some cases, detect exhaustiveness.

```kotlin
var result = when (card) {
    Suit.SPADES -> "The swords of a soldier"
    Suit.CLUBS -> "Weapons of war"
    Suit.DIAMONDS -> "Money for this art"
    else -> "The shape of my heart"
}
```

Further reading: https://blog.kotlin-academy.com/enum-vs-sealed-class-which-one-to-choose-dc92ce7a4df5

### Scala

Scala enums are also built on objects.

```scala
package com.crell.poker {
    object Suit extends Enumeration {
        type Suit = Value
        val HEARTS, DIAMONDS, CLUBS, SPADES = Value
    }
}

// ...
object Main extends App {
    import com.crell.poker.Suit._

    var s = CLUBS

    // Iteration
    Suit.values foreach println
}
```

They can carry values, including multiple values, which must be pre-set and not vary by instance.  They also can support methods that way, although my Scala-fu is not strong enough to know if my syntax here is entirely correct. :-)

```scala
object Suit extends Enumeration {
    protected case class Val(abbrev: String) extends super.Val {
        def color: String = abbrev.match {
            case Suit.HEARTS => "Red"
            case Suit.DIAMONDS => "Red"
            case Suit.CLUBS => "Black"
            case Suit.SPADES => "Black"
        }
    }
    type Suit = Value
    val HEARTS = Val("H")
    val DIAMONDS = Val("D")
    val CLUBS = Val("C")
    val SPADES = Val("S")
}
```

Further reading: https://www.scala-lang.org/api/current/scala/Enumeration.html

## Summary

Folded into a convenient table, a feature summary looks like this:

| Language          | C/C++  | Java | Python | Typescript | Haskell | F# (Union) | F# (Enum) |  C#  | Swift | Rust | Kotlin (Enums) | Kotlin (Sealed) | Scala
|-------------------|--------|------|--------|------------|---------|------------|-----------|------|-------|------|----------------|-----------------|-------
| Unit values       | No     | No   | No     | No         | Yes     | Yes        | No        | No   | Yes   | Yes  | Yes            | Ish?            | Yes
| Int values        | Yes    | Yes  | Yes    | Yes        | No      | No         | Yes       | Yes  | Yes   | Yes  | Yes            | Ish?            | Yes
| String values     | No     | No   | Yes    | Yes        | No      | No         | No        | No   | Yes   | No   | Yes            | Ish?            | Yes
| Associated values | No     | No   | No     | No         | Yes     | No         | No        | No   | Yes   | Yes  | No             | Yes             | No
| Methods           | No     | Yes  | Yes    | No         | No?     | No         | No        | Ish  | Yes   | Yes  | Yes            | Yes             | Yes
| Type checked      | Ish    | Yes  | No     | Yes        | Yes     | No         | Ish       | Yes  | Yes   | Yes  | Yes            | Yes             | Yes?
| Iterable          | No     | Yes  | Yes    | No         | No      | No         | No        | Yes  | Yes   | No   | Yes            | No              | No

In terms of overall capability, Swift appears to have the edge with Rust a very close second.  However, Rust also seems to have more powerful associated values ability (tuples or structs), and the usefulness of iterating enum types is debatable.  I'm going to call it a qualified tie between those two in raw expressive power.

## Analysis

Broadly speaking, I would separate the languages into a few categories:

* **Fancy Constants**: C, Typescript, F#
* **Fancy Objects**: Python, Java, C#, Scala
* **Algebraic Data Types**: Haskell, Swift, Rust, Kotlin

While they are superficially similar, and often use the same terminology, they approach the problem from different ways.  The Fancy Constants languages are offering a syntactic convenience, but little else.  Often they get compiled away at runtime, and their type checking may be incomplete.

The Fancy Objects languages take that a step further and offer methods on enum types, which offers a centralized place to put a switch, match, or whatever branching syntax for RTTI.  That is helpful, and helps with data modeling in ways that Fancy Constants do not.  If the methods need to vary by enum type more than just a little, though, you run into some contortions and may find yourself better off with normal objects and interfaces.

The main differentiator for ADT languages, as I'm using them here, is that they can be parameterized with different values.  That offers another layer again of potential functionality and data modeling.  It also becomes a natural and easy way to implement Monads in user space, and Haskell, Swift, and Rust all do exactly that in their core libraries, particularly for Maybe/Optional.  That makes them an extremely robust way to handle data modeling in your application, and to "make invalid states unrepresentable," which is an excellent feature if you can get it.

The downside is that once you start parameterizing enum values, you no longer get a guarantee that a Club is a Club is a Club.  They may well be two different Clubs.  The implementation details here around equality (a tricky subject in the best of circumstances) are the devil's hiding place.  The other catch is that, as far as I can tell, no language with parameterized enum values lets you get at them easily without doing pattern matching.  Depending on your use case that may be no big deal or may be a deal-breaker.  In practice, I think it largely comes down to how easy the syntax is for pattern matching; Of all things I'd say Haskell is the nicest here, followed by Swift, then Rust.  (Or possibly Rust then Swift, depending on your tastes.  Rust gets very tricky when you have struct-parameterized enums.)

## For PHP

As far as borrowing ideas for a PHP implementation, it seems silly at this point to not go all the way to ADT support if possible.  "If possible" being the operative phrase, as PHP also lacks relevant features that some ADT languages use for their enums, such as pattern matching.  In practice, I believe the only question is between Fancy Objects and ADTs.

The main implementation questions would be:

### Backed by objects, or a new primitive?

In practice, I see little reason to not build on objects.  They're already there, and if we want methods and associated data and type checking then you're already 90% to objects.  Most of the ADT implementations above build on objects, either implicitly or explicitly.

### Enums, Unions, or Sealed classes?

The majority of ADT/fancy class languages here use a dedicated enum syntax of some variety.  The exceptions are Kotlin's sealed classes and Haskell's union types, which are not strictly speaking enums but close enough.

It has been suggested that the addition of a `typedef` to PHP to allow pre-definition of union types (already added in PHP 8.0, coming soon) would be "close enough" to enums to render any further effort unnecessary.  That is true up to a point; the main limitation would be no way to enforce that all of the unioned types are type compatible: that they share an interface that makes it possible to type against them properly.  It would also make the degenerate case of "I just want a closed list of options" more verbose.

```php
typedef Suit = Hearts | Diamonds | Clubs | Spades;

// This is just weird
interface SuitInterface {}

class Hearts implements SuitInterface {}
class Diamonds implements SuitInterface {}
class Clubs implements SuitInterface {}
class Spades implements SuitInterface {}
```

Sealed classes would look essentially the same, give or take some syntax.  Ideally we want an approach that "scales" cleanly from the basic case to the highly complex case.  I think a properly designed dedicated Enum syntax is the best way to achieve that (even if it's just syntactic sugar that decomposes to the same as above).

### What types of values?

There are largely 3 types of "single associated value" that enums can have: Unit (just the enum itself), Int only, or any primitive.  The latter two get tricky if implemented as objects in PHP, unless it's just still-more sugar.  I think Kotlin has the right idea here, though; Rather than giving an enum value its own direct primitive equivalent, make that a constant associated value.  That is, no, you cannot define "Diamonds" to be equal to 3, but you can define "Diamonds" to be an object with a single property whose value is always 6.

### Equality

This one gets really tricky, as equality is troublesome at the best of times.

https://twitter.com/Tojiro/status/823286025535393792
