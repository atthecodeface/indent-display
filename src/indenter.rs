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
use std::cell::RefCell;
use std::rc::Rc;

use crate::IndentedOptions;

//a Type aliases
type IOResult = std::result::Result<(), std::io::Error>;
type FmtResult = std::result::Result<(), std::fmt::Error>;
type RrcRoot<'a, Opt> = Rc<RefCell<Root<'a, Opt>>>;
type RrcInner<'a, Opt> = Rc<RefCell<Inner<'a, Opt>>>;

//a Root
//ti Root
/// The root of the indenter - this is used as an Rc/RefCell
/// so that it can be accessed by any depth of display node
struct Root<'a, Opt: IndentedOptions<'a>> {
    /// The underlying Write object that provides the output method
    fmt: &'a mut (dyn std::io::Write + 'a),
    /// The options the indenter was created with
    options: &'a Opt,
    /// Set if a newline is pending
    pending_newline: bool,
    /// Boolean set to true if at the start of a line - so if real
    /// characters are to be output, the appropriate indentation must
    /// be performed first
    sol: bool,
    /// The basic indentation string to be used per level, unless
    /// explicit per-level indents are provided
    ind: &'a str,
    /// The current stack of indentation strings and the depth
    /// associated with them; this is an empty vector if a single
    /// indent string is used.
    subind: Vec<(usize, &'a str)>,
    /// The current depth of indentation
    depth: usize,
}

//ii Root
impl<'a, Opt: IndentedOptions<'a>> Root<'a, Opt> {
    //fi new
    /// Create a new [Root] of indentation, with a base indent string
    fn new(fmt: &'a mut (dyn std::io::Write + 'a), ind: &'a str, options: &'a Opt) -> Self {
        let subind = Vec::new();
        Self {
            fmt,
            options,
            pending_newline: false,
            sol: true,
            ind,
            subind,
            depth: 0,
        }
    }

    //fi push_indent
    /// Push a new indentation onto the stack - depth is presumably +1
    /// on the current depth; if the indentation string provided is
    /// Some then the indentation at this point will use this instead
    /// of the base indentation
    fn push_indent(&mut self, depth: usize, ind: Option<&'a str>) {
        self.pending_newline = true;
        if let Some(ind) = ind {
            self.subind.push((self.depth, ind));
        }
        self.depth = depth;
    }

    //fi pop_indent
    /// Pop the indent from the stack down to a new depth (which is
    /// presumably self.depth-1)
    ///
    /// This may involve popping the top of subind, if that is for the
    /// indentation depth being popped
    fn pop_indent(&mut self, depth: usize) {
        self.pending_newline = true;
        if let Some((d, _)) = self.subind.last() {
            if *d == depth {
                self.subind.pop();
            }
        }
        self.depth = depth;
    }

    //fi output_newline
    /// Output a newline *if required*
    ///
    /// After the newline the output will be at the start of a line;
    /// hence `sol` is set, and any characters to output afterwards
    /// will require the appropriate indent
    fn output_newline(&mut self) -> IOResult {
        self.pending_newline = false;
        if self.sol {
            Ok(())
        } else {
            self.sol = true;
            self.fmt.write_all(b"\n")
        }
    }

    //fi output_indent
    /// Output the current indentation
    ///
    /// After the newline the output will be at the start of a line;
    /// hence `sol` is set, and any characters to output afterwards
    /// will require the appropriate indent
    fn output_indent(&mut self) -> IOResult {
        let sublen = self.subind.len();
        let mut s = 0;
        for i in 0..self.depth {
            if s < sublen {
                if self.subind[s].0 == i {
                    self.fmt.write_all(self.subind[s].1.as_bytes())?;
                    s += 1;
                } else {
                    self.fmt.write_all(self.ind.as_bytes())?;
                }
            } else {
                self.fmt.write_all(self.ind.as_bytes())?;
            }
        }
        Ok(())
    }

    //fi output_str
    /// Output a string that contains no newlines
    ///
    /// An empty string requires no output
    ///
    /// If there is data to output and the last output left it at the
    /// start of a line then indentation is required first to the
    /// current depth
    fn output_str(&mut self, s: &str) -> IOResult {
        // If there is nothing to show then must not indent - it may
        // be that the indent changes before there is something to
        // output
        if s.is_empty() {
            return Ok(());
        }
        if self.pending_newline {
            self.output_newline()?;
        }
        if self.sol {
            self.output_indent()?;
        }
        self.sol = false;
        self.fmt.write_all(s.as_bytes())
    }

    //fi complete
    /// Invoked by the last stack frame being dropped; tidy up the
    /// output
    ///
    /// No errors can be returned (this is in Drop)
    fn complete(&mut self) {
        if self.pending_newline {
            let _ = self.output_newline();
        }
    }

    //zz All done
}

//ii Debug for Root
impl<'a, Opt: IndentedOptions<'a>> std::fmt::Debug for Root<'a, Opt> {
    //fp fmt
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Root[sol:{} depth: {}, ind:{:?}]",
            self.sol, self.depth, self.ind
        )
    }
}

//ii Write for Root
impl<'a, Opt: IndentedOptions<'a>> std::fmt::Write for Root<'a, Opt> {
    //fp write_str
    /// Perform the actual write operation, providing a write!
    /// capability (etc) for Root
    ///
    /// The string to be written must be split into individual lines
    /// which are then output
    ///
    /// output_newline is invoked *between* every line of output
    /// i.e. for every newline character in the input string
    fn write_str(&mut self, s: &str) -> FmtResult {
        let mut output_newline = false;
        for line in s.split('\n') {
            if output_newline {
                if self.output_newline().is_err() {
                    return Err(std::fmt::Error);
                }
            }
            if self.output_str(line).is_err() {
                return Err(std::fmt::Error);
            }
            output_newline = true;
        }
        Ok(())
    }
}

//a Inner
//ti Inner
/// This represents an indentation 'stack frame', including the uppermost stack frame.
///
/// All the stack frames for the same indenter refer to the same
/// [Root], which is created by the uppermost stack frame creation.
///
/// When a subframe is created it clones the [Root], and sets its parent appropriately
///
/// When the subframe is dropped it informs the [Root] and resets the
/// state back to that of its parent.
#[derive(Debug)]
pub struct Inner<'a, Opt: IndentedOptions<'a>> {
    root: RrcRoot<'a, Opt>,
    parent: Option<RrcInner<'a, Opt>>,
    depth: usize,
}

//ii Drop for Inner
impl<'a, Opt: IndentedOptions<'a>> Drop for Inner<'a, Opt> {
    //fi drop
    /// Invoked automatically by Rust when the stack frame goes out of
    /// scope allowing the indentation to revert to that prior to the
    /// creation of this stack frame
    fn drop(&mut self) {
        if let Some(parent) = &self.parent {
            let depth = parent.borrow().depth;
            self.root.borrow_mut().pop_indent(depth);
        } else {
            self.root.borrow_mut().complete();
        }
    }
}

//ii Inner
impl<'a, Opt: IndentedOptions<'a>> Inner<'a, Opt> {
    //fi root
    /// Create a root stack frame - which has no parent and is at depth 0
    fn root(root: RrcRoot<'a, Opt>) -> RrcInner<'a, Opt> {
        Rc::new(RefCell::new(Self {
            root: root,
            parent: None,
            depth: 0,
        }))
    }

    //fi subnode
    /// Create a subnode of this stack frame, with an optional
    /// depth-specific indentation string
    fn subnode(s: &Rc<RefCell<Self>>, ind: Option<&'a str>) -> RrcInner<'a, Opt> {
        let root = s.borrow().root.clone();
        let parent = Some(s.clone());
        let depth = s.borrow().depth + 1;
        root.borrow_mut().push_indent(depth, ind);
        Rc::new(RefCell::new(Self {
            root,
            parent,
            depth,
        }))
    }

    //fi pop
    /// Create a subnode of this stack frame, with an optional
    /// depth-specific indentation string
    fn pop(self) -> Option<RrcInner<'a, Opt>> {
        if let Some(parent) = &self.parent {
            Some(parent.clone())
        } else {
            None
        }
    }

    //fi take_parent
    /// Provided this [Inner] has a parent (panic otherwise), return
    /// that parent by deconstructing this
    ///
    /// This node must not be borrowed elsewhere for this to work,
    /// as it cannot be deconstructed if so
    fn take_parent(s: Rc<RefCell<Self>>) -> RrcInner<'a, Opt> {
        assert!(s.borrow().parent.is_some());
        match Rc::try_unwrap(s) {
            Err(_) => {
                panic!("Indent was multiply borrowed");
            }
            Ok(x) => x.into_inner().pop().unwrap(),
        }
    }

    //zz All done
}

//a Indenter
//tp Indenter
/// The public face of the library, this is the type that must be
/// created to use the [IndentedDisplay] trait
///
/// This utilizes a [std::fmt::Write] formatter as its output, a base
/// indent string that is used for all levels of indentation (unless
/// overridden individually by indentation frames), and an options
/// structure that contains options that may be interrogated by the
/// implementation of IndentedDisplay
pub struct Indenter<'a, Opt: IndentedOptions<'a>> {
    node: RrcInner<'a, Opt>,
}

//ip Indenter
impl<'a, Opt: IndentedOptions<'a>> Indenter<'a, Opt> {
    //fp new
    /// Create a new [Indenter], to be used with types that implement
    /// the IndentedDisplay trait; this specifies the formatter, the
    /// base indentation string, and the options for the indentation
    pub fn new(fmt: &'a mut (dyn std::io::Write + 'a), s: &'a str, options: &'a Opt) -> Self {
        let r = Rc::new(RefCell::new(Root::new(fmt, s, options)));
        let node = Inner::root(r);
        Self { node }
    }

    //fp sub
    /// Create a new subframe of the [Indenter] using its base
    /// indentation for this indentation level; this is invoked by the
    /// `indent` function in an [IndentedDisplay] trait implementation
    /// to create subframes of indentation. The subframe is removed
    /// from the indentation output stack when it is *dropped*, so it
    /// must either go out of scope or be explicitly dropped.
    pub fn sub(&self) -> Self {
        let node = Inner::subnode(&self.node, None);
        Self { node }
    }

    //fp push
    /// Create a new subframe of the [Indenter] using a specific string,
    /// indentation for this indentation level; this is invoked by the
    /// `indent` function in an [IndentedDisplay] trait implementation
    /// to create subframes of indentation. The subframe is removed
    /// from the indentation output stack when it is *dropped*, so it
    /// must either go out of scope or be explicitly dropped.
    ///
    /// Currently the string must outlive the Indenter - usually this
    /// means it is static.
    pub fn push(&self, s: &'a str) -> Self {
        let node = Inner::subnode(&self.node, Some(s));
        Self { node }
    }

    //dp pop
    /// Pop this subframe and return its parent
    pub fn pop(self) -> Self {
        let node = Inner::take_parent(self.node);
        Self { node }
    }

    //fp options
    /// Borrow the options used to invoke the [Indenter].
    ///
    /// This may be invoked by the
    /// `indent` function in an [IndentedDisplay] trait implementation
    /// to determine the setting of indentation options that may affect its output.
    pub fn options(&self) -> &Opt {
        &self.node.borrow().root.borrow().options
    }

    //zz All done
}

//ip Write
impl<'a, Opt: IndentedOptions<'a>> std::fmt::Write for Indenter<'a, Opt> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.node.borrow().root.borrow_mut().write_str(s)
    }
}
