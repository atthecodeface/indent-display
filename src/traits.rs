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

@file    traits.rs
@brief   Basic traits for the indenter
 */

//a Imports
use crate::Indenter;

//a Traits
//tt IndentedOptions
pub trait IndentedOptions<'a>: Sized + 'a {}

//tt IndentedDisplay
pub trait IndentedDisplay<'a, Opt: IndentedOptions<'a>> {
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result;
}

//tt DefaultIndentedDisplay
pub trait DefaultIndentedDisplay: std::fmt::Display {}
