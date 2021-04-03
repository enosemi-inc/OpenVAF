mod apple_base;
//mod linux_musl_base;
mod linux_base;
mod windows_msvc_base;
//mod android_base;
//mod crt_objects;

use crate::host_triple;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum LinkerFlavor {
    Ld,
    Ld64,
    Msvc,
}

/// Everything Mun knows about a target.
/// Every field must be specified, there are no default values.
#[derive(PartialEq, Clone, Debug)]
pub struct Target {
    /// Target triple to pass to LLVM
    pub llvm_target: String,

    /// String to use as the `target_endian` `cfg` variable.
    pub target_endian: String,

    /// String to use as the `target_pointer_width` `cfg` variable.
    pub target_pointer_width: String,

    /// Width of c_int type
    pub target_c_int_width: String,

    /// The name of the OS
    pub target_os: String,

    /// The name of the environment
    pub target_env: String,

    /// The name of the vendor
    pub target_vendor: String,

    /// The name of the architecture. For example "x86" or "x86_64"
    pub arch: String,

    /// [Data layout](http://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM.
    pub data_layout: String,

    /// Linker flavor
    pub linker_flavor: LinkerFlavor,

    /// Optional settings
    pub options: TargetOptions,
}

/// Optional aspects of target specification.
#[derive(PartialEq, Clone, Debug)]
pub struct TargetOptions {
    /// True if this is a built-in target
    pub is_builtin: bool,

    /// Default CPU to pass to LLVM. Corresponds to `llc -mcpu=$cpu`. Defaults to "generic".
    pub cpu: String,

    /// Default target features to pass to LLVM. These features will *always* be passed, and cannot
    /// be disabled even via `-C`. Corresponds to `llc -mattr=$features`.
    pub features: String,

    /// Whether the target toolchain is like Windows
    pub is_like_windows: bool,
    pub is_like_msvc: bool,
}

impl Default for TargetOptions {
    fn default() -> Self {
        TargetOptions {
            is_builtin: false,
            cpu: "generic".to_string(),
            features: "".to_string(),
            is_like_windows: false,
            is_like_msvc: false,
        }
    }
}
#[derive(Error, Debug)]
pub enum LoadTargetError {
    #[error("target not found: {0}")]
    BuiltinTargetNotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type TargetResult = Result<Target, String>;

macro_rules! supported_targets {
    ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
        $ ( mod $ module; ) +

        /// List of supported targets
        const TARGETS: &[&str] = &[$($($triple),+),+];

        fn load_specific(target: &str) -> Result<Target, LoadTargetError> {
            match target {
                $(
                    $($triple)|+ => {
                        let mut t = $module::target()
                            .map_err(LoadTargetError::Other)?;
                        t.options.is_builtin = true;

                        tracing::debug!("got builtin target: {:?}", t);
                        Ok(t)
                    },
                )+
                    _ => Err(LoadTargetError::BuiltinTargetNotFound(
                        format!("Unable to find target: {}", target)))
            }
        }

        pub fn get_targets() -> impl Iterator<Item = String> {
            TARGETS.iter().filter_map(|t| -> Option<String> {
                load_specific(t)
                    .and(Ok(t.to_string()))
                    .ok()
            })
        }
    }
}

supported_targets!(
    ("x86_64-apple-darwin", x86_64_apple_darwin),
    ("x86_64-pc-windows-msvc", x86_64_pc_windows_msvc),
    ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
 //   ("x86_64-unknown-linux-musl", x86_64_unknown_linux_musl), TODO MUSL SUPPORT
);

impl Target {
    pub fn search(target_triple: &str) -> Result<Target, LoadTargetError> {
        load_specific(target_triple)
    }

    pub fn host_target() -> Result<Target, LoadTargetError> {
        Self::search(host_triple())
    }
}