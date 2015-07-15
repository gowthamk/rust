fn main () {
  /*
  let v = vec! [1,2,3];
  v.push(4);
  //Error: Cannot borrow immutable local variable `v` as mutable.
  */
  let mut v = vec! [1,2,3];
  let v1 = &mut v;
}
