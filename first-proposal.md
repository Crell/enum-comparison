
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

There are largely 3 types of "single associated value" that enums can have: Unit (just the enum itself), Int only, or any primitive.  The latter two get tricky if implemented as objects in PHP, unless it's just still-more sugar.  I think Kotlin has the right idea here, though; Rather than giving an enum value its own direct primitive equivalent, make that a constant associated value.  That is, no, you cannot define "Diamonds" to be equal to 3, but you can define "Diamonds" to be an object with a single property whose value is always 3.

### Equality

This one gets really tricky, as equality is troublesome at the best of times.  Already in PHP, when two objects are "equal" is somewhat confusing.

```php
$a = new Foo();
$b = new Foo();

print $a == $a . PHP_EOL;   // True
print $a === $b . PHP_EOL;  // False
print $a == $b . PHP_EOL;   // True
```

Depending on the implementation this may end up doing all kinds of weird thing that may or may not make sense.

### Proposal

To simplify an implementation for PHP, I would recommend the following:

* Enums are, mostly, sugar for abstract classes and inheritance.  Yes, inheritance has its issues, but in this context its behavior is actually desireable.
* Enum values may only be object instances.  They may not map to primitives.  However, a well-designed `__toString()` method can get us 80% of the way there with 10% of the effort, which seems like a good trade-off.  Plus it emphasizes enum-as-unit, which I would argue is preferable from a modeling perspective in the majority case.
* We allow some conceptual leakage in return for not needing pattern matching.  This is another "80% of the benefit for 10% of the effort" case, and is consistent with Kotlin's approach anyway.

For the simple case:

```php
enum Suit {
    case Hearts;
    case Diamonds;
    case Clubs;
    case Spades;
}
```

is equivalent to:

```php
class Suit extends Enum implements IteratorAggregate  {
    public static Hearts $Hearts;
    public static Diamonds $Diamonds;
    public static Clubs $Clubs;
    public static Spades $Spades;

    public function getIterator(): array {
        return [static::Hearts, static::Diamonds, static::Clubs, static::Spades];
    }
}

class Hearts extends Suit {}
class Diamonds extends Suit {}
class Clubs extends Suit {}
class Spades extends Suit {}

Suit::$Hearts = new Hearts();
Suit::$Diamonds = new Diamonds();
Suit::$Clubs = new Clubs();
Suit::$Spades = new Spades();
```

And assumes a base class like:

```php
abstract class Enum {

    public static abstract function values(): array

    public function __toString(): string {
        // Via reflection:
        // If this object has no properties, return the name of the class.
        // If it has one property, return that property.
        // If it has multiple, concatenate them in lexical order and return that.
        // This method may of course be overridden.
    }
}
```

The automatic `IteratorAggregate` implementation happens if and only if all of the defined `case`s is a unit.  If any of them may have associated data, then it is not generated and the enum is not iterable.  (This is consistent with Swift.)

For enum members that can have one or more constant values applied to them:

```php
enum Suit (public string $abbrev) {
    case Hearts("H");
    case Diamonds("D");
    case Clubs("C");
    case Spades("S");
}
```

That is equivalent to:

```php
class Suit extends Enum {
    public static Hearts $Hearts;
    public static Diamonds $Diamonds;
    public static Clubs $Clubs;
    public static Spades $Spades;

    public function __construct(public string $abbrev) {}

    public function getIterator(): array {
        return [static::Hearts, static::Diamonds, static::Clubs, static::Spades];
    }
}

class Hearts extends Suit {}
class Diamonds extends Suit {}
class Clubs extends Suit {}
class Spades extends Suit {}

Suit::$Hearts = new Hearts("H");
Suit::$Diamonds = new Diamonds("D");
Suit::$Clubs = new Clubs("C");
Suit::$Spades = new Spades("S");
```

Of note, if enum members are going to have constant values, they *must* all have the same constant values, and they *must* be defined in the enum itself using compact constructor syntax.  Whether the value is public or private is then an explicit decision of the enum author.  The enum author *may not*, however, directly implement a constructor.

If *any one* of the enum members defines its own parameters, however, that means the following:

1. The enum itself may not define any constant values.
2. No iteration is defined.
3. It follows that any other enum members must be either units (no data at all) or have their own parameters.

```php
enum Optional {
    case None;
    case Some(public mixed $value);
}
```

Is equivalent to:

```php
class Optional extends Enum {
    public static None $None;

    public static function Some(public mixed $value);
}

class None extends Optional {}

class Some extends Optional {
    public function __construct(public mixed $value);
}

Optional::$None = new None();
```

Again, it's the implementer's choice if the associated values are public or private.

In all three cases, both the enum and its members may have methods, with the following restrictions:

1. Any methods defined on an individual member *must* be defined on the enum itself, either fully or abstract.
2. It therefore follows that all members *must* have an implementation of the same methods.

This logic is lifted directly from Kotlin.


```php
enum Optional {
    case None {
        public function valueOr(mixed $default): mixed {
            return $default;
        }
    };

    case Some(protected mixed $value) {
        public function valueOr(mixed $default): mixed {
            return $this->value;
        }
    };

    abstract public function valueOr(mixed $default): mixed;

    public function bind(callable $c): static {
        if ($this instanceof Optional::None) {
            return $this;
        }
        return static::Some($c($this->value));
    }
}
```

Is equivalent to:

```php
class Optional extends Enum {
    public static None $None;

    public static function Some(protected mixed $value);

    public function bind(callable $c): static {
        if ($this instanceof Optional::None) {
            return $this;
        }
        return static::Some($c($this->value));
    }
}

class None extends Optional {
    public function valueOr(mixed $default): mixed {
        return $default
    }
}

class Some extends Optional {
    public function __construct(public mixed $value);

    public function valueOr(mixed $default): mixed {
        return $this->value;
    }
}

Optional::$None = new None();
```

As demonstrated above, whether a method is single-instanced and checks for the enum type internally or is split into separate methods is up to the implementer.

The main enum may also implement interfaces if desired, in which case all the usual rules about inheritance and interfaces apply.

##### Implications

* Most existing plumbing for classes and objects applies to enums if needed.  They behave in a mostly predictable fashion.  That includes being able to autoload the enum itself.  If and when named parameters are adopted, creating a new associated-data enum member will automatically support that syntax.
* While value-bound enums are not technically supported, the approach above combind with the `__toString()` method offer a "close enough" equivalent.
* In the unit case, because member instances are singletons they will always `===` each other.  In the associated data case, they will not.
* Whether associated data should be public or private is punted to the implementer.  Given PHP's general approach to visibility that seems the best option.  On the plus side, it means any improvements made to PHP's visibility in the future should "just work" on enums as well.  (Eg, if asymmetric visibility were ever added in a constructor-promotion-compatible way, they would become available to enums.)
* Enum members do *not* support arbitrary member variables beyond those defined through constructor promotion.  That is a deliberate limitation because if you need that much functionality, you don't want an enum.  You want a traditional class.
* Presumably Enums and Members could support attributes.  What you would do with them I do not know, but I see no reason to not allow them.
* The existence of a base `Enum` class gives us a place to put future extensions or functionality, such as a default `__serialize`, `__debugInfo`, or other such magic method implementations.

#### Open questions

* As shown here, enum members end up as public static class properties.  That is suboptimal.  Ideally they would be be write-once and exposed as though they were constants.  This may be something that can be special cased in the engine.
* It's unclear how to handle `instanceof` and references to the class.  If implemented as pure sugar, that means members do become their own stand-alone objects and you'd do `$c instance of Diamonds`.  That is suboptimal.  It would be preferable to always make the name scoped to the enum itself, ie, `$c instanceof Suit::Diamonds`.  However, it's unclear how that would interact with the `Diamonds()` member itself or with the static factory method in the case of associated data.  This requires more investigation and possibly engine trickry.
* If implemented as pure sugar, a side effect of this approach is that members become accessible as stand-alone classes to instantiate.  That is not desireable.  Ideally the actual implementation would be more robust and disallow that somehow.
