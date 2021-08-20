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

//a Documentation
/*!

# Indent

This is a fairly simple indented display system designed for standard
library (not simply core) applications.

```
use indent_display::{Indenter, NullOptions, DefaultIndentedDisplay, IndentedDisplay};
let mut stdout = std::io::stdout();
let mut ind = Indenter::new(&mut stdout, "  ", &NullOptions {});
"Not indented\n".indent(&mut ind);
{
    let mut sub = ind.sub();
    "Indented once with two spaces\n".indent(&mut ind);
}
{
    let mut sub = ind.push("...");
    "Indented once with three dots\n".indent(&mut ind);
    {
        let mut sub = sub.push("***");
        "Indented with three dots and three stars\nAnd so is this".indent(&mut ind);
    }
    {
        let mut sub = sub.sub();
        "Indented with three dots and two spaces stars\nAnd so is this".indent(&mut ind);
    }
}
"Not indented\n".indent(&mut ind);
```

!*/

//a Imports
mod defaults;
mod indenter;
mod test;
mod traits;
mod types;

//a Exports
pub use traits::{DefaultIndentedDisplay, IndentedDisplay, IndentedOptions};
pub use types::NullOptions;
// pub use defaults::{};
pub use indenter::Indenter;
