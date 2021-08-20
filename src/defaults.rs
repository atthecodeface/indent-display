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

//a Imports
use crate::{DefaultIndentedDisplay, IndentedDisplay, IndentedOptions, Indenter, NullOptions};

//a DefaultIndentedDisplay implementation
//ti IndentedDisplay for DefaultIndentedDisplay
impl<'a, O: IndentedOptions<'a>, T: DefaultIndentedDisplay> IndentedDisplay<'a, O> for T {
    fn indent(&self, ind: &mut Indenter<'a, O>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(ind, "{}", self)
    }
}

//ti DefaultIndentedDisplay for base types
impl DefaultIndentedDisplay for u8 {}
impl DefaultIndentedDisplay for u16 {}
impl DefaultIndentedDisplay for u32 {}
impl DefaultIndentedDisplay for u64 {}
impl DefaultIndentedDisplay for u128 {}
impl DefaultIndentedDisplay for usize {}
impl DefaultIndentedDisplay for i8 {}
impl DefaultIndentedDisplay for i16 {}
impl DefaultIndentedDisplay for i32 {}
impl DefaultIndentedDisplay for i64 {}
impl DefaultIndentedDisplay for i128 {}
impl DefaultIndentedDisplay for isize {}
impl DefaultIndentedDisplay for &str {}
impl DefaultIndentedDisplay for String {}
impl<'a, Opt: IndentedOptions<'a>, T: IndentedDisplay<'a, Opt>> IndentedDisplay<'a, Opt> for [T] {
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "[\n")?;
        {
            let mut sub = f.sub();
            for x in self.iter() {
                x.indent(&mut sub)?;
                write!(sub, ",\n")?;
            }
        }
        write!(f, "]\n")
    }
}

//a NullOptions
//ti IndentedOptions
impl IndentedOptions<'_> for NullOptions {}
