/*
 * Can there be two simultaneous mutable references? No. A mutable
 * borrow effectively preempts any other reference to the object until
 * its scope ends.
 */
fn example4() {
  let mut x = vec!["Hello", "world"];
  let y = &mut x[0];
  let z = &mut x[1]; // This is illegal
  /* Error: cannot borrow `x` as mutable more than once at a time 
   * (This means, at this point of time, a borrowed mutable
   * reference is live). Note: previous borrow of `x` occurs above;
   * the mutable borrow prevents subsequent moves, borrows, or
   * modification of `x` until the borrow ends. */
}
/*
 * This example is a converse of example 1.
 */
fn example3() {
  let mut x = vec!["Hello", "world"];
  let y = &mut x[0];
  let z = &x[1]; // This is illegal
  /* Cannot borrow `x` as immutable because it is also borrowed as
   * mutable (This means, at this point of time, a borrowed mutable
   * reference is live). Note: previous borrow of `x` occurs above;
   * the mutable borrow prevents subsequent moves, borrows, or
   * modification of `x` until the borrow ends */
}
/*
 * This example is an inverse of example 1.
 */
fn example2() {
  let mut x = vec!["Hello", "world"];
  x.push("foo"); // This is legal
  let y = &x[0]; 
  /* Althought push borrows `x` via a mutable reference, the borrowing
   * ends before the immutable reference `y` is created.
   * Note: `x` is guaranteed to be live after call to `push`
   * concludes; borrowing ends when the scope of `push` ends, but the
   * deallocation does not happen. Also, recall that there is no
   * `free`.
   */
}
fn example1() {
  let mut x = vec!["Hello", "world"];
  let y = &x[0];
  x.push("foo"); // This is illegal.
  /* Cannot borrow `x` as mutable because it is also borrowed as
   * immutable (This means, at this point of time, a borrowed immutable
   * reference is live). Note: previous borrow of `x` occurs above; the
   * immutable borrow prevents subsequent moves or mutable borrows of
   * `x` until the borrow ends. */
}
fn main() {
  let mut x = vec!["Hello", "world"];
  x.push("foo"); // This is legal.
  /* push borrows a mutable reference, 
     and x is declared mutable. */
}
