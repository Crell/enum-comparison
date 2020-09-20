# PHP RFC: Enumerations and Algebraic Data Types

* Date: 2020-09-19
* Author: Larry Garfield (larry@garfieldtech.com), Ilija Tovilo (tovilo.ilija@gmail.com)
* Status: Draft
* Target Version: PHP 8.1
* Implementation: TBD

## Introduction

This RFC introduces Enumerations to PHP. Specifically, it introduces what are variously called "Algebraic Data Types", "tagged unions", or simply "enumerations" depending on the language. This capability offers greatly expanded support for data modeling, custom type definitions, and monad-style behavior. Enums enable the modeling technique of "make invalid states unrepresentable", which leads to more robust code with less need for exhaustive testing.

Many languages have support for enumerations of some variety. A [survey we conducted of various languages](https://github.com/Crell/enum-comparison) found that they could be categorized into three general groups: Fancy Constants, Fancy Objects, and full Algebraic Data Types. For this implementation we opted to implement full Algebraic Data Types, as that offers the most robust set of functionality while also degrading gracefully to simpler use cases. (Or it progressively enhances to more complex use cases, depending on your point of view.)

The specific implementation here draws inspiration primarily from Swift, Rust, and Kotlin, but is not (nor is it intended as) a perfect 1:1 port of any of them.

The most popular case of enumerations is `boolean`, which is an enumerated type with legal values `true` and `false`. This RFC allows developers to define their own arbitrarily robust enumerations.

## Proposal

### Basic enumerations

This RFC introduces a new language construct, `enum`. Enums are similar to classes, and share the same namespaces as classes, interfaces, and traits. They are also autoloadable the same way. An Enum defines a new type, which has a fixed, limited number of possible legal values.

```php
enum Suit {
  case Hearts;
  case Diamonds;
  case Clubs;
  case Spades;
}
```

This declaration creates a new enumerated type named `Suit`, which has four and only four legal values: `Suit::Hearts`, `Suit::Diamonds`, `Suit::Clubs`, and `Suit::Spades`. Variables may be assigned to one of those legal values. A function may be type checked against an enumerated type, in which case only values of that type may be passed.

```php
$val = Suit::Diamonds;

function pick_a_card(Suit $suit) { ... }

pick_a_card($val);       // OK
pick_a_card(Suit:Clubs); // OK
pick_a_card('Spades');   // throws TypeError
```

In the simple case, multiple cases may be defined on a single line. The following is semantically equivalent to the definition above.

```php
enum Suit {
  case Hearts, Diamonds, Clubs, Spades;
}
```

An Enumeration may have one or more `case` definitions, with no maximum, although at least one is required.

Cases are not backed by a primitive value. That is, `Suit::Hearts` is not equal to 0. Instead, each case is backed by a singleton object of that name. That means that:

```php
$a = Suit::Spades;
$b = Suit::Spades;

$a === $b; // true


$a instanceof Suit;         // true
$a instanceof Suit::Spades; // true
```

[Note to Ilija: The last line there is the tricksy one we haven't figured out.]

Each Case class includes a default `__toString()` implementation that returns the name of the Case as a string, without the Enum type. That is:

```php
print Suit::Clubs; 
// prints "Clubs", not "Suit::Clubs".
```

That function may be overridden if desired. (See below.)

[To Ilija: Do we want this part or not?  I only thought of it while writing this. I don't know if it's good or bad.]

Enumerated type Cases may be used in union type definitions. For example:

```php
function gimmie_red_card(Suit::Hearts|Suit::Diamonds $card) { ... }
```

### Enumerated Case Methods

As both Enum Types and Enum Cases are implemented using classes, they may take methods. The Enum Type may also implement an interface, which all Cases must then fulfill, directly or indirectly.

```php
interface Colorful {
  public function color(): string;
}

enum Suit implements Colorful {
  case Hearts {
    public function color(): string {
      return "Red";
    }
  };  // Note the semi-colon here!
  
  case Diamonds {
    public function color(): string {
      return "Red";
    }
  };
  
  case Clubs {
    public function color(): string {
      return "Black";
    }
  };
  
  case Spades {
    public function color(): string {
      return "Black";
    }
  };
  
  public function shape(): string {
    return "Rectangle";
  }
}

function paint(Colorful $c) { ... }

paint(Suit::Clubs);  // Works
```

In this example, all four Enum cases will have a method `shape` inherited from `Suit`, and will all have their own method `color`, which they implement themselves. Case methods may be arbitrarily complex, and function the same as any other method. Additionally, magic methods such as `__toString` and friends may also be implemented and will behave like a normal method on an object. The one exception is `__construct`, which it not permitted. (See below.)

Enum Cases may not implement interfaces themselves.

Static methods on Cases are not supported. Static methods on the Enum Type are supported.

[Ilija: We haven't discussed static methods at all. This is what makes the most sense to me at the moment but we can easily revisit this. I'm flexible.)

Inside a method on a Case, The `$this` variable is defined and refers to the Case instance. (That is mainly useful with Associated Values. See below.)

(Note that in this case it would be a better data modeling practice to also define a `SuitColor` Enum Type with values Red and Black and return that instead. However, that would complicate this example.)

The above hierarchy is logically similar to the following class structure:

```php
interface Colorful {
  public function color(): string;
}

abstract class Suit implements Colorful {
  public function shape(): string {
    return "Rectangle";
  }
}

class Hearts extends Suit {
  public function color(): string {
    return "Red";
  }
}

class Diamonds extends Suit {
  public function color(): string {
    return "Red";
  }
}

class Clubs extends Suit {
  public function color(): string {
    return "Black";
  }
}
  
class Spades extends Suit {
  public function color(): string {
    return "Black";
  }
}
```

### Associated Values

Enumerated Cases may optionally include associated values. An associated value is one that is associated with an instance of a Case. If a Case has associated values, it will **not** be implemented as a singleton. Each instance of the Case will then be its own object instance, so will not === another instance.

Associated values are defined using constructor property promotion.

```php
enum Distance {
    case Kilometers(public int $num);
    case Miles(public int $num);
}

$my_walk = Distance::Miles(500);
// Named parameters work like any other function call.
$next_walk = Distance::Miles(num: 500);

print $my_walk->num; // prints "500"

$my_walk === $next_walk; // FALSE!
```

Enum Cases may not implement a full constructor. However, they may list parameters that will be auto-promoted to properties using constructor promotion. The visibility modifier is required. Cases may not implement properties other than promoted properties.

An Enum Case that supports Associated Values is called an Associable Case. An Enum Case that does not have Associated Values is called a Unit Case. An Enumerated Type may consist of any combination of Associable and Unit Cases.

The Enum Type itself may not define associated values. Only a Case may do so.

Associated values are always read-only, both internally to the class and externally. Therefore, making them public does not pose a risk of 3rd party code modifying them inadvertently. They may, however, have attributes associated with them like any other property.

Use cases that would require more complete class functionality (arbitrary properties, custom constructors, mutable properties, etc.) should be implemented using traditional classes instead.

### Match expressions

When dealing with Unit Cases, `match` expressions offer a natural and convenient way to branch logic depending on the enum value. Since every instance of a Unit Case is a singleton, it will always pass an identity check. Therefore:

```php
$val = Suit::Diamonds;

$str = match ($val) {
    Suit::Spades => "The swords of a soldier",
    Suit::Clubs => "Weapons of war",
    Suit::Diamonds => "Money for this art",
    default => "The shape of my heart",
}
```

That is not true when dealing with Associable Cases. Therefore, an alternate version of `match` is included. When `match` is suffixed with `type`, it will perform an `instanceof` check instead of an identity check.

```php
$val = Distance::Miles(500);

$str = match type ($val) {
    Distance::Kilometers => "Traveling $val->num km",
    Distance::Miles => "Traveling $val->num miles",
}
```

[Ilija, your thoughts on this?]

### Examples

Below are a few examples of Enums in action.

#### Maybe

The (in)famous Maybe Monad can be implemented like this:

```php
enum Maybe {
  // This is a Unit Case.
  case None {
    public function bind(callable $f) {
      return $this;
    }
  };
    
  // This is an Associable Case.
  case Some(private mixed $value) {
    // Note that the return type can be the Enum itself, thus restricting the return
    // value to one of the enumerated types.
    public function bind(callable $f) {
      // $f is supposed to return a Maybe itself.
      return $f($this->value);
    }
  };

  // This method is available on both None and Some.
  public function value(): mixed {
    // Still need to sort out match() for this to make sense.
    return match type ($this) {
        Optional::None => throw new Exception(),
        Optional::Some => $this->val,
    };
  }
}
```

#### State machine

Enums make it straightforward to express finite state machines.

```php
enum OvenStatus {

  case Off {
    public function turnOn() { return OvenStatus::On; }
  };
  
  case On {
    public function turnOff() { return OvenStatus::Off; }
    public function idle() { return OvenStatus::Idle; }
  };
  
  case Idle {
    public function on() { return OvenStatus::On; }
  };
}
```

In this example, the oven can be in one of three states (Off, On, and Idling, meaning the flame is not on but it will turn back on when it detects it needs to). However, it can never go from Off to Idle or Idle to Off; it must go through On state first. That means no tests need to be written or code paths defined for going from Off to Idle, because it's literally impossible to even describe that state.

(Additional methods are of course likely in a real implementation.)

#### Single Associable Enums

Because all properties on an Enum are readonly, they offer a back-door way to create immutable objects.

```php
enum Point {
  case ThreeD(public $x, public $x, public $z);
}

$p = Point::ThreeD(x: 3, y: 5, z: 7);

print $p->y; // prints 5
$p->z = 9;   // throws an Error of some kind, TBD.
```

This is not a specific design goal of the implementation, but a potentially useful side effect.


## Backward Incompatible Changes

"enum" becomes a language keyword, with the usual potential for naming conflicts with existing global constants.

## Future Scope

### Case enumeration

In some languages, it is possible to enumerate all possible values of an Enum Type. For now that functionality is not implemented, but it may be in the future. It would be limited to the case where the Enum Type contains only Unit Values. (That limitation exists in other languages as well.)

### Pattern matching

Most languages that have an equivalent of associated values also support pattern matching as a way to extract values from the Enum Case. Pattern matching allows for a single `match` branch to match on, for example, "any Foo::Bar instance where one of its two parameters is the number 5, and the other is extracted out into a variable to be used on the right."  While a powerful feature in its own right, we believe that at this time it is not an MVP for useful Enumerations. It also has a large number of potential gotchas and complications all on its own, making it worthy of its own stand-alone RFC and development effort.

For now, matching against the Enum Case and accessing properties directly (something not supported in most ADT-supporting languages) is "good enough" and has mostly self-evident semantics based on existing PHP patterns.

## Voting

This is a simple yes/no vote to include Enumerations. 2/3 required to pass.

## References

[Survey of enumerations supported by various languages, conducted by Larry](https://github.com/Crell/enum-comparison}
