---
sidebar_position: 1
---

# Generics

Write reusable, type-safe code with generics.

## Generic Functions

Functions that work with any type:

```forge
// Identity function - returns value unchanged
fn identity<T>(value: T) -> T {
    return value;
}

var num = identity(42);         // T = int
var str_val = identity("hello"); // T = str

// Swap two values
fn swap<A, B>(a: A, b: B) -> (B, A) {
    return (b, a);
}

var (x, y) = swap(1, "one");  // (str, int)

// Generic with multiple same-type params
fn make_pair<T>(first: T, second: T) -> (T, T) {
    return (first, second);
}

var pair = make_pair(1, 2);  // (int, int)
```

## Generic Classes

Classes with type parameters:

```forge
// Stack - Last In First Out
class Stack<T> {
    var items: list<T>;
    
    fn new() -> Stack<T> {
        return Stack { items: [] };
    }
    
    fn push(self, item: T) -> void {
        self.items.push(item);
    }
    
    fn pop(self) -> Option<T> {
        if self.items.length() == 0 { return Option.None; }
        var top = self.items.last();
        self.items.pop();
        return Option.Some(top);
    }
    
    fn peek(self) -> Option<T> {
        if self.items.length() == 0 { return Option.None; }
        return Option.Some(self.items.last());
    }
    
    fn is_empty(self) -> bool {
        return self.items.length() == 0;
    }
    
    fn size(self) -> int {
        return self.items.length();
    }
}

// Usage
var int_stack: Stack<int> = Stack.new();
int_stack.push(1);
int_stack.push(2);
var top = int_stack.pop();  // Option.Some(2)

var str_stack: Stack<str> = Stack.new();
str_stack.push("hello");
```

## Generic Interfaces

Interfaces with type parameters:

```forge
// Container interface
interface Container<T> {
    fn get(self) -> T;
    fn set(self, value: T) -> void;
}

// Implementation
class Box<T> implements Container<T> {
    var value: T;
    
    fn get(self) -> T { self.value }
    fn set(self, value: T) -> void { self.value = value; }
}

// Comparable interface
interface Comparable<T> {
    fn compare(self, other: T) -> int;
    fn eq(self, other: T) -> bool;
    fn lt(self, other: T) -> bool;
    fn gt(self, other: T) -> bool;
}

class Point implements Comparable<Point> {
    var x: int;
    var y: int;
    
    fn compare(self, other: Point) -> int {
        if self.x < other.x { return -1; }
        if self.x > other.x { return 1; }
        return 0;
    }
    
    fn eq(self, other: Point) -> bool { self.compare(other) == 0 }
    fn lt(self, other: Point) -> bool { self.compare(other) < 0 }
    fn gt(self, other: Point) -> bool { self.compare(other) > 0 }
}
```

## Type Constraints

Restrict generic types:

```forge
// Constrain to types implementing an interface
fn sort<T: Comparable<T>>(items: list<T>) -> list<T> {
    // Can use compare, lt, gt methods
    var sorted = items.clone();
    // ... sorting logic using T's compare method
    return sorted;
}

// Multiple constraints
fn process<T>(value: T) -> str 
where T: Displayable + Clone {
    return value.to_str();
}

// Built-in type constraints
fn add<T>(a: T, b: T) -> T 
where T: Add {
    return a + b;
}
```

## Generic Type Aliases

Create aliases for generic types:

```forge
// Result type alias
type Result<T> = Result<T, str>;

// Function type aliases
type Predicate<T> = fn(T) -> bool;
type Transform<T> = fn(T) -> T;
type Comparator<T> = fn(T, T) -> int;
type Reducer<T, A> = fn(A, T) -> A;

// Usage
fn filter<T>(items: list<T>, pred: Predicate<T>) -> list<T> { ... }
fn map<T, U>(items: list<T>, f: Transform<T, U>) -> list<U> { ... }
```

## Higher-Kinded Types

Generic type constructors:

```forge
// Functor interface
interface Functor<F<T>> {
    fn map<A, B>(self, f: fn(A) -> B) -> F<B>;
}

// Option as Functor
implement Functor<Option> for Option {
    fn map<A, B>(self, f: fn(A) -> B) -> Option<B> {
        return match self {
            Option.Some(a) => Option.Some(f(a)),
            Option.None => Option.None
        };
    }
}

// Usage
var num: Option<int> = Option.Some(42);
var str: Option<str> = num.map(fn(n) { return "Number: {n}"; });
```

## Generic Enums

Enums with type parameters:

```forge
// Option - nullable value
enum Option<T> {
    Some(value: T),
    None
}

// Result - error handling
enum Result<T, E> {
    Ok(value: T),
    Err(error: E)
}

// Linked list
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

// Tree
enum Tree<T> {
    Leaf(T),
    Node(Tree<T>, Tree<T>)
}

// Usage
fn find<T>(list: List<T>, target: T) -> Option<int> {
    // ...
}
```

## Associated Types

Types associated with interfaces:

```forge
interface Iterator {
    type Item;
    
    fn next(self) -> Option<Self.Item>;
}

class Range implements Iterator {
    type Item = int;
    
    var current: int;
    var end: int;
    
    fn next(self) -> Option<int> {
        if self.current >= self.end {
            return Option.None;
        }
        var value = self.current;
        self.current = self.current + 1;
        return Option.Some(value);
    }
}
```

## Trait Objects

Dynamic dispatch with generics:

```forge
// Store different types implementing same interface
var shapes: list<Box<dyn Displayable>> = [];
shapes.push(Box.new(Circle { radius: 5.0 }));
shapes.push(Box.new(Rectangle { width: 4.0, height: 6.0 }));

for shape in shapes {
    println(shape.to_str());
}
```

## Best Practices

1. **Use meaningful type parameter names** - `T` for single, `K, V` for key/value
2. **Constrain when needed** - Don't over-constrain
3. **Document type requirements** - Explain what types must implement
4. **Prefer composition over complex generics** - Keep it simple

```forge
// Good type parameter names
fn zip<A, B>(a: list<A>, b: list<B>) -> list<(A, B)> { ... }
fn map<K, V>(map: map<K, V>, f: fn(K, V) -> V) -> map<K, V> { ... }

// Clear constraints
fn sort<T>(items: list<T>) -> list<T> 
where T: Comparable<T> {
    // ...
}
```

## Next Steps

- **[Pattern Matching](./pattern-matching)** - Exhaustive matching
- **[Interfaces](./interfaces)** - Define contracts
- **[Error Handling](./error-handling)** - Result and Option
