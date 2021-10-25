pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        // strtok<'a, 'b>(&'a mut &'b      str) -> &'b      str
        // strtok        (&'a mut &'static str) -> &'static str
        let z = &mut x; // &'x mut -> &'until-z mut
        println!("{}", *z);
        // until-z: borrow of x stops here
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}

// T (e.g. 'static) is a subtype of U (e.g. 'a) --if-- T is at least as useful as U
//
// covariance
//  fn foo(&'a str) {}
//  let x: &'a str
//  foo (&'a str)
//  foo (&'static str)
//  x = &'a str
//  x = &'static str
//  &'static str --more useful than-- &'a str
//  'static <: 'a --then-- &'static T <: &'a T
//
// contravariance
//  fn foo(Fn(&'a str) -> ())
//  let x: Fn(&'a str) -> ()
//  foo(fn(&'static str)) {})
//  Fn(&'a str) --more useful than-- Fn(&'static str)
//  'static <: 'a --then-- Fn(&'a T) <: Fn(&'static T)
//
// invariance
//  fn foo(s: &mut &'a str, x: &'a str) {
//    *s = x;
//  }
//  let mut x: &'static str = "hello world";
//  let z = String::new();
//  foo(&mut x, &z);
//    // foo(&mut &'a      str, &'a str)
//    // foo(&mut &'static str, &'a str)
//  drop(z);
//  println!("{}", x);
