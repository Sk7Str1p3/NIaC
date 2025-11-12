//! ## Panic handler
//!
//! Provides a compact, human-friendly panic report
//! formatter by implementing [`PanicMessage`]. Simple and
//! colorful.

use std::thread;

use color_eyre::owo_colors::OwoColorize;
use color_eyre::section::PanicMessage;

/// A type representing an error report for a panic.
/// ### Possible output:
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
pub(crate) struct Panic;

impl PanicMessage for Panic {
    fn display(
        &self,
        info: &std::panic::PanicHookInfo<'_>,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            "Unexpected error occured! The application panicked (crashed)."
                .red()
                .bold()
        )?;

        let payload = info
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| info.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<???>");

        write!(f, "Message:   ")?;
        writeln!(f, "{}", payload.blue())?;

        if let Some(loc) = info.location() {
            writeln!(f, "Location: {{")?;
            writeln!(f, "   file:   {}", loc.file().purple())?;
            writeln!(f, "   line:   {}", loc.line().purple())?;
            writeln!(f, "   column: {}", loc.column().purple())?;
            writeln!(f, "}}")?;
        } else {
            writeln!(
                f,
                "Location: {}:{}:{}",
                "src/{unknown}.rs".purple(),
                "??".purple(),
                "??".purple()
            )?;
        }

        write!(f, "Thread:    ")?;
        writeln!(
            f,
            "{} (id: {})",
            thread::current().name().unwrap_or("{unknown}").magenta(),
            // TODO: remove then #67939 become stable
            format!("{:?}", thread::current().id())
                .trim_start_matches("ThreadId(")
                .trim_end_matches(")")
                .magenta()
        )?;

        Ok(())
    }
}
