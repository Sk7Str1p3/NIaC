//! ## Errors
//! Module for configuring and installing pretty, colorized
//! error and panic reporting using the
//! [`color-eyre`](../../color_eyre/index.html) crate.
use color_eyre::Result;
use color_eyre::config::{
    HookBuilder,
    Theme
};
use color_eyre::owo_colors::Style;

mod panic;

/// Initializes error and panic reporting.
///
/// Installs a color_eyre [`HookBuilder`] with
/// custom panic message,
/// [`tracing`](../../tracing/index.html) messages,
/// backtraces, etc...
///
/// Function should be first called in `main()`, so all
/// errors and panics are reported with the desired
/// formatting.
///
/// ### Possible Output of Error
#[doc = r#"
<pre>
 Error:
    0: <font color=red>could not set the provided `Theme` via `color_spantrace::set_theme` globally as another was already set</font>
    1: <font color=red>could not set the provided `Theme` globally as another was already set</font>
 Location:
    <font color=purple>/home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/color-eyre-0.6.5/src/config.rs</font>:<font color=purple>729</font>

   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   0: <b><font color=green>bootstrap::dirs_setup</font></b> with <i><font color=cyan>target</font></i>="unknown"
      at <b><font color=purple>src/main.rs</font></b>:<b><font color=purple>19</font></b>
        17 │
        18 │     let (flake, output) = {
        <font color=red>19 >         let span = tracing::info_span!("dirs_setup", target = "unknown");</font>
        20 │         let _guard = span.enter();
        21 │

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                <font color=blue>⋮ 4 frames hidden ⋮</font>                               
   5: <b><font color=green>&lt;E as eyre::context::ext::StdError>::ext_report</font></b><font color=gray>::h6c726ad9e2ee0184</font>
      at <b><font color=purple>/home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/eyre-0.6.12/src/context.rs</font></b>:<b><font color=purple>26</font></b>
        24 │             D: Display + Send + Sync + 'static,
        25 │         {
        <font color=red>26 >             Report::from_msg(msg, self)</font>
        27 │         }
        28 │     }
                                <font color=blue>⋮ 3 frames hidden ⋮</font>                               
   9: <b><font color=green>bootstrap::error::init</font></b><font color=gray>::he1fa55383da4651a</font>
      at <b><font color=purple>/home/user/Documents/project/bootstrap/src/error/mod.rs</font></b>:<b><font color=purple>58</font></b>
        56 │         .capture_span_trace_by_default(true)
        57 │         .display_location_section(true)
        <font color=red>58 >         .install()</font>
        59 │ }
  10: <b><font color=green>bootstrap::main::</font></b><font color=gray>h99469d586afec4d8</font>
      at <b><font color=purple>/home/user/Documents/project/src/main.rs</font></b>:<b><font color=purple>78</font></b>
        76 │         };
        77 │         tracing::info!("{} {}", "OUT:".blue().bold(), output.path().display());
        <font color=red>78 >         error::init()?;</font>
        79 │         (flake, output)
        80 │     };
  11: <font color=yellow>core::ops::function::FnOnce::call_once</font><font color=gray>::hc629f3c4e976641f</font>
      at <b><font color=purple>/nix/store/fxm41f1z8aj7m9z6f5rlapwi6khvh87k-rust-default-1.90.0/lib/rustlib/src/rust/library/core/src/ops/function.rs</font></b>:<b><font color=purple>253</font></b>
       251 │     /// Performs the call operation.
       252 │     #[unstable(feature = "fn_traits", issue = "29625")]
       <font color=red>253 >     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;</font>
       254 │ }
       255 │
  12: <font color=yellow>std::sys::backtrace::__rust_begin_short_backtrace</font><font color=gray>::hf3a24543d6d7dda9</font>
      at <b><font color=purple>/nix/store/fxm41f1z8aj7m9z6f5rlapwi6khvh87k-rust-default-1.90.0/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs</font></b>:<b><font color=purple>158</font></b>
       156 │     F: FnOnce() -> T,
       157 │ {
       <font color=red>158 >     let result = f();</font>
       159 │
       160 │     // prevent this frame from being tail-call optimised away
                                <font color=blue>⋮ 15 frames hidden ⋮</font>                              

</pre>
"#]
/// ### Possible Output of Panic
#[doc = r#"
<pre>
 <b><font color=red>Unexpected error occured! The application panicked (crashed).</font></b>
 Message:   <font color=blue>test</font>
 Location: {
    file:   <font color=magenta>src/log/mod.rs</font>
    line:   <font color=magenta>16</font>
    column: <font color=magenta>5</font>
 }
 Thread:    <font color=magenta>main</font> (id: <font color=magenta>1</font>)


  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                                <font color=blue>⋮ 8 frames hidden ⋮</font>                               
   9: <b><font color=green>bootstrap::log::init</font></b><font color=gray>::he4c315af3dbad96e</font>
      at <b><font color=purple>/home/user/Documents/project/src/log/mod.rs</font></b>:<b><font color=purple>16</font></b>
        14 │ #[inline]
        15 │ pub fn init() -> Result<()> {
        <font color=red>16 >     panic!("test");</font>
        17 │     let subscriber = tracing_subscriber::fmt()
        18 │         .event_format(format::Tracer)
  10: <b><font color=green>bootstrap::main</font></b><font color=gray>::h99469d586afec4d8</font>
      at <b><font color=purple>/nix/persist/user/home/Sk7Str1p3/Documents/_nixos/helpers/bootstrap/src/main.rs</font></b>:<b><font color=purple>15</font></b>
        13 │ fn main() -> Result<()> {
        14 │     error::init()?;
        <font color=red>15 >     log::init()?;</font>
        16 │     sigint::init()?;
        17 │ 
  11: <font color=yellow>core::ops::function::FnOnce::call_once</font><font color=gray>::hc629f3c4e976641f</font>
      at <b><font color=purple>/nix/store/fxm41f1z8aj7m9z6f5rlapwi6khvh87k-rust-default-1.90.0/lib/rustlib/src/rust/library/core/src/ops/function.rs</font></b>:<b><font color=purple>253</font></b>
       251 │     /// Performs the call operation.
       252 │     #[unstable(feature = "fn_traits", issue = "29625")]
       <font color=red>253 >     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;</font>
       254 │ }
       255 │ 
  12: <font color=yellow>std::sys::backtrace::__rust_begin_short_backtrace</font><font color=gray>::hf3a24543d6d7dda9</font>
      at <b><font color=purple>/nix/store/fxm41f1z8aj7m9z6f5rlapwi6khvh87k-rust-default-1.90.0/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs</font></b>:<b><font color=purple>158</font></b>
       156 │     F: FnOnce() -> T,
       157 │ {
       <font color=red>158 >     let result = f();</font>
       159 │ 
       160 │     // prevent this frame from being tail-call optimised away
                                <font color=blue>⋮ 15 frames hidden ⋮</font>                              
</pre>
"#]
#[inline]
pub fn init() -> Result<()> {
    let theme = Theme::dark()
        .error(Style::new().red())
        .file(Style::new().purple().bold())
        .hidden_frames(Style::new().bright_blue())
        .crate_code(Style::new().green().bold())
        .dependency_code(Style::new().yellow())
        .help_info_note(Style::new().bright_green())
        .active_line(Style::new().bright_red())
        .spantrace_target(Style::new().green().bold())
        .line_number(Style::new().purple().bold());

    HookBuilder::new()
        .panic_message(panic::Panic)
        .theme(theme)
        .capture_span_trace_by_default(true)
        .display_location_section(true)
        .install()
}
