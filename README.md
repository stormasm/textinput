
This is a fork of [gpui-component](https://github.com/huacnlee/gpui-component)

With just the ui crate which has been renamed to *ti*.

The reason it has been renamed it so that gpui applications can use both the
ui crate that comes with Zed along with another crate called ti.

Leaving the crate the same name as ui would cause a conflict.

The main functionality we are trying to show here is the input widget.

Other widgets include:

- scroll
- button
- colors
- event
- history
- icon
- indicator
- styled
- theme

For more details of this code being used please see the workspace branch of [textinput-test](https://github.com/stormasm/textinput-test).

If you just want the textinput functionality and do not care about the workspace functionality then the *main* branch
should probably suffice.
