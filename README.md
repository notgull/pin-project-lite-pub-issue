```
$ cargo build
   Compiling exporter-crate v0.1.0 (/home/jtnunley/Programming/Projects/pin-project-lite-breakage-demo/exporter-crate)
   Compiling user-crate v0.1.0 (/home/jtnunley/Programming/Projects/pin-project-lite-breakage-demo/user-crate)
error: missing documentation for a struct
 --> user-crate/src/lib.rs:9:1
  |
9 | wrapper! { pub }
  | ^^^^^^^^^^^^^^^^
  |
note: the lint level is defined here
 --> user-crate/src/lib.rs:5:11
  |
5 | #![forbid(missing_docs)]
  |           ^^^^^^^^^^^^
  = note: this error originates in the macro `$crate::__pin_project_reconstruct` which comes from the expansion of the macro `wrapper` (in Nightly builds, run with -Z macro-backtrace for more info)

error: missing documentation for a struct field
 --> user-crate/src/lib.rs:9:1
  |
9 | wrapper! { pub }
  | ^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `$crate::__pin_project_reconstruct` which comes from the expansion of the macro `wrapper` (in Nightly builds, run with -Z macro-backtrace for more info)

error: missing documentation for a method
 --> user-crate/src/lib.rs:9:1
  |
9 | wrapper! { pub }
  | ^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `$crate::__pin_project_struct_make_proj_method` which comes from the expansion of the macro `wrapper` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `user-crate` (lib) due to 4 previous errors
```