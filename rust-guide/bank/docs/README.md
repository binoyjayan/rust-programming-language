# Bank project

## Ownership rules

- Every value is 'owned' by a single variable, struct, vector at a time
- Reassigning the value to another variable, passing it to a function,
  etc moves the value

## Borrowing

- You can create many read-only references to a value that exist at the same time
- You cannot move a value while a reference to it exist
- You can make a mutable reference to a value only if there are no
  read-only references currently in use.
  Only one mutable ref to a value can exist at one time
- You cannot mutate a value through the owner when any ref (mutable or immutable)
  to the value exists
- Some types of values are copied instead of moved

## Lifetimes

- When a variable goes out of scope, the value owned by it is dropped
- Values cannot be dropped if there are active refs to it
- References to a value cannot outlive the value they refer to
