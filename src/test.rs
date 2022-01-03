/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    indent.rs
@brief   An indented display system
 */

//a Test
#[cfg(test)]
mod test {
    use crate::{IndentedDisplay, IndentedOptions, Indenter};
    use std::fmt::{Display, Write};
    struct Options {
        ind_leaves: bool,
    }
    impl IndentedOptions<'_> for Options {}
    struct Leaf<T: Display + Sized> {
        t: T,
    }
    impl<T: Display + Sized> Leaf<T> {
        fn new(t: T) -> Self {
            Self { t }
        }
    }
    impl<T: Display + Sized> Display for Leaf<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.t)
        }
    }

    impl<'a, T: Display + Sized> IndentedDisplay<'a, Options> for Leaf<T> {
        fn indent(&self, ind: &mut Indenter<'_, Options>) -> std::fmt::Result {
            write!(ind, "Leaf")?;
            let mut ind = ind.sub();
            write!(&mut ind, "{}", self)
        }
    }

    struct Joint<T: Display + Sized> {
        left: Option<Box<Joint<T>>>,
        mid: Leaf<T>,
        right: Option<Box<Joint<T>>>,
    }
    impl<T: Display + Sized> Joint<T> {
        fn new(l: Leaf<T>) -> Self {
            Self {
                left: None,
                mid: l,
                right: None,
            }
        }
        fn set_left(mut self, left: Joint<T>) -> Self {
            self.left = Some(Box::new(left));
            self
        }
        fn set_right(mut self, right: Joint<T>) -> Self {
            self.right = Some(Box::new(right));
            self
        }
    }
    impl<'a, T: Display + Sized> IndentedDisplay<'a, Options> for Joint<T> {
        fn indent(&self, ind: &mut Indenter<'_, Options>) -> std::fmt::Result {
            if let Some(left) = &self.left {
                let mut ind = ind.push("  <");
                left.indent(&mut ind)?;
            }
            if ind.options().ind_leaves {
                write!(ind, "--")?;
                self.mid.indent(ind)?;
            } else {
                write!(ind, "--{}", self.mid)?;
            }
            if let Some(right) = &self.right {
                let mut ind = ind.push("  >");
                right.indent(&mut ind)?;
            }
            Ok(())
        }
    }
    #[test]
    fn test_tree() {
        use crate::IndentedDisplay;
        let tree = Joint::new(Leaf::new(4u64))
            .set_left(Joint::new(Leaf::new(3u64)).set_left(Joint::new(Leaf::new(2u64))))
            .set_right(
                Joint::new(Leaf::new(8u64))
                    .set_left(Joint::new(Leaf::new(5u64)))
                    .set_right(Joint::new(Leaf::new(10u64))),
            );

        let mut r = Vec::new();
        let mut ind = Indenter::new(&mut r, "    ", &Options { ind_leaves: false });
        let output = r###"  <  <--2
  <--3
--4
  >  <--5
  >--8
  >  >--10
"###;
        tree.indent(&mut ind).unwrap();
        drop(ind);

        let r = std::str::from_utf8(&r).unwrap();
        println!("{}", r);
        assert_eq!(r, output);

        let mut r = Vec::new();
        let mut ind = Indenter::new(&mut r, "__", &Options { ind_leaves: true });
        let output = r###"  <  <--Leaf
  <  <__2
  <--Leaf
  <__3
--Leaf
__4
  >  <--Leaf
  >  <__5
  >--Leaf
  >__8
  >  >--Leaf
  >  >__10
"###;
        tree.indent(&mut ind).unwrap();
        drop(ind);

        let r = std::str::from_utf8(&r).unwrap();
        println!("{}", r);
        assert_eq!(r, output);
    }

    #[test]
    fn test_types() {
        use crate::IndentedDisplay;
        let mut r = Vec::new();
        let mut ind = Indenter::new(&mut r, "    ", &Options { ind_leaves: false });
        let output = r###"[
    1,
    2,
    3,
]
[
    4,
    5,
    6,
]
banana apple pear
"###;
        [1usize, 2, 3].indent(&mut ind).unwrap();
        vec![4isize, 5, 6].indent(&mut ind).unwrap();
        "banana ".indent(&mut ind).unwrap();
        "apple ".indent(&mut ind).unwrap();
        format!("pear\n").indent(&mut ind).unwrap();

        drop(ind);
        let r = std::str::from_utf8(&r).unwrap();
        println!("{}", r);
        assert_eq!(r, output);
    }
}
