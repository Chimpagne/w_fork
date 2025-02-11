//! Common module with common used structures across different
//! commands.

// NOTE: A lot of this code depends on feature flags.
// To not go crazy with annotations, some lints are disabled for the whole
// module.
#![allow(dead_code, unused_imports, unused_variables)]

use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;

use anyhow::{bail, Result};
#[cfg(feature = "sys")]
use wasmer::sys::*;
use wasmer::*;

#[cfg(feature = "compiler")]
use wasmer_compiler::CompilerConfig;

use wasmer::Engine;

#[derive(Debug, clap::Parser, Clone, Default)]
/// The WebAssembly features that can be passed through the
/// Command Line args.
pub struct WasmFeatures {
    /// Enable support for the SIMD proposal.
    #[clap(long = "enable-simd")]
    pub simd: bool,

    /// Disable support for the threads proposal.
    #[clap(long = "disable-threads")]
    pub disable_threads: bool,

    /// Deprecated, threads are enabled by default.
    #[clap(long = "enable-threads")]
    pub _threads: bool,

    /// Enable support for the reference types proposal.
    #[clap(long = "enable-reference-types")]
    pub reference_types: bool,

    /// Enable support for the multi value proposal.
    #[clap(long = "enable-multi-value")]
    pub multi_value: bool,

    /// Enable support for the bulk memory proposal.
    #[clap(long = "enable-bulk-memory")]
    pub bulk_memory: bool,

    /// Enable support for all pre-standard proposals.
    #[clap(long = "enable-all")]
    pub all: bool,
}

#[derive(Debug, Clone, clap::Parser, Default)]
/// The compiler options
pub struct RuntimeOptions {
    /// Use Singlepass compiler.
    #[cfg(feature = "singlepass")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "llvm")]
        "llvm", 
        #[cfg(feature = "v8")]
        "v8", 
        #[cfg(feature = "cranelift")]
        "cranelift", 
        #[cfg(feature = "wamr")]
        "wamr", 
        #[cfg(feature = "wasmi")]
        "wasmi"
    ]))]
    singlepass: bool,

    /// Use Cranelift compiler.
    #[cfg(feature = "cranelift")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "llvm")]
        "llvm", 
        #[cfg(feature = "v8")]
        "v8", 
        #[cfg(feature = "singlepass")]
        "singlepass", 
        #[cfg(feature = "wamr")]
        "wamr", 
        #[cfg(feature = "wasmi")]
        "wasmi"
    ]))]
    cranelift: bool,

    /// Use LLVM compiler.
    #[cfg(feature = "llvm")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "cranelift")]
        "cranelift", 
        #[cfg(feature = "v8")]
        "v8", 
        #[cfg(feature = "singlepass")]
        "singlepass", 
        #[cfg(feature = "wamr")]
        "wamr", 
        #[cfg(feature = "wasmi")]
        "wasmi"
    ]))]
    llvm: bool,

    /// Use the V8 runtime.
    #[cfg(feature = "v8")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "cranelift")]
        "cranelift", 
        #[cfg(feature = "llvm")]
        "llvm", 
        #[cfg(feature = "singlepass")]
        "singlepass", 
        #[cfg(feature = "wamr")]
        "wamr", 
        #[cfg(feature = "wasmi")]
        "wasmi"
    ]))]
    v8: bool,

    /// Use WAMR.
    #[cfg(feature = "wamr")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "cranelift")]
        "cranelift", 
        #[cfg(feature = "llvm")]
        "llvm", 
        #[cfg(feature = "singlepass")]
        "singlepass", 
        #[cfg(feature = "v8")]
        "v8", 
        #[cfg(feature = "wasmi")]
        "wasmi"
    ]))]
    wamr: bool,

    /// Use the wasmi runtime.
    #[cfg(feature = "wasmi")]
    #[clap(long, conflicts_with_all = &Vec::<&str>::from_iter([
        #[cfg(feature = "cranelift")]
        "cranelift", 
        #[cfg(feature = "llvm")]
        "llvm", 
        #[cfg(feature = "singlepass")]
        "singlepass", 
        #[cfg(feature = "v8")]
        "v8", 
        #[cfg(feature = "wamr")]
        "wamr"
    ]))]
    wasmi: bool,

    /// Enable compiler internal verification.
    ///
    /// Available for cranelift, LLVM and singlepass.
    #[clap(long)]
    enable_verifier: bool,

    /// LLVM debug directory, where IR and object files will be written to.
    ///
    /// Only available for the LLVM compiler.
    #[clap(long)]
    llvm_debug_dir: Option<PathBuf>,

    #[clap(flatten)]
    features: WasmFeatures,
}

impl RuntimeOptions {
    pub fn get_rt(&self) -> Result<RuntimeType> {
        #[cfg(feature = "cranelift")]
        {
            if self.cranelift {
                return Ok(RuntimeType::Cranelift);
            }
        }

        #[cfg(feature = "llvm")]
        {
            if self.llvm {
                return Ok(RuntimeType::LLVM);
            }
        }

        #[cfg(feature = "singlepass")]
        {
            if self.singlepass {
                return Ok(RuntimeType::Singlepass);
            }
        }

        #[cfg(feature = "wamr")]
        {
            if self.wamr {
                return Ok(RuntimeType::Wamr);
            }
        }

        #[cfg(feature = "v8")]
        {
            if self.v8 {
                return Ok(RuntimeType::V8);
            }
        }

        #[cfg(feature = "wasmi")]
        {
            if self.wasmi {
                return Ok(RuntimeType::Wasmi);
            }
        }

        // Auto mode, we choose the best compiler for that platform
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "cranelift", any(target_arch = "x86_64", target_arch = "aarch64")))] {
                Ok(RuntimeType::Cranelift)
            } else if #[cfg(all(feature = "singlepass", any(target_arch = "x86_64", target_arch = "aarch64")))] {
                Ok(RuntimeType::Singlepass)
            } else if #[cfg(feature = "llvm")] {
                Ok(RuntimeType::LLVM)
            } else if #[cfg(feature = "v8")] {
                Ok(RuntimeType::V8)
            } else if #[cfg(feature = "wamr")] {
                Ok(RuntimeType::Wamr)
            } else if #[cfg(feature = "wasmi")] {
                Ok(RuntimeType::Wasmi)
            } else {
                bail!("There are no available runtimes for your architecture");
            }
        }
    }

    pub fn get_store(&self) -> Result<Store> {
        #[cfg(feature = "compiler")]
        #[allow(clippy::needless_return)]
        {
            let target = Target::default();
            return self.get_store_for_target(target);
        }

        #[cfg(not(feature = "compiler"))]
        {
            let engine = self.get_engine()?;
            Ok(Store::new(engine))
        }
    }

    pub fn get_engine(&self) -> Result<Engine> {
        #[cfg(feature = "compiler")]
        #[allow(clippy::needless_return)]
        {
            let target = Target::default();
            return self.get_engine_for_target(target);
        }
        #[cfg(not(feature = "compiler"))]
        {
            Ok(match self.get_rt()? {
                #[cfg(feature = "v8")]
                RuntimeType::V8 => v8::V8::new().into(),
                #[cfg(feature = "wamr")]
                RuntimeType::Wamr => wamr::Wamr::new().into(),
                #[cfg(feature = "wasmi")]
                RuntimeType::Wasmi => wasmi::Wasmi::new().into(),
                _ => unreachable!(),
            })
        }
    }

    #[cfg(feature = "compiler")]
    /// Get the enaled Wasm features.
    pub fn get_features(&self, mut features: Features) -> Result<Features> {
        if !self.features.disable_threads || self.features.all {
            features.threads(true);
        }
        if self.features.disable_threads && !self.features.all {
            features.threads(false);
        }
        if self.features.multi_value || self.features.all {
            features.multi_value(true);
        }
        if self.features.simd || self.features.all {
            features.simd(true);
        }
        if self.features.bulk_memory || self.features.all {
            features.bulk_memory(true);
        }
        if self.features.reference_types || self.features.all {
            features.reference_types(true);
        }
        Ok(features)
    }

    /// Gets the Store for a given target.
    #[cfg(feature = "compiler")]
    pub fn get_store_for_target(&self, target: Target) -> Result<Store> {
        let rt = self.get_rt()?;
        let engine = self.get_engine_for_target_and_rt(target, &rt)?;
        let store = Store::new(engine);
        Ok(store)
    }

    #[cfg(feature = "compiler")]
    pub fn get_engine_for_target(&self, target: Target) -> Result<Engine> {
        let rt = self.get_rt()?;
        self.get_engine_for_target_and_rt(target, &rt)
    }

    #[cfg(feature = "compiler")]
    fn get_engine_for_target_and_rt(&self, target: Target, rt: &RuntimeType) -> Result<Engine> {
        match rt {
            RuntimeType::V8 => {
                #[cfg(feature = "v8")]
                return Ok(wasmer::v8::V8::new().into());
                #[allow(unreachable_code)]
                {
                    anyhow::bail!("The `v8` engine is not enabled in this build.")
                }
            }
            RuntimeType::Wamr => {
                #[cfg(feature = "wamr")]
                return Ok(wasmer::wamr::Wamr::new().into());
                #[allow(unreachable_code)]
                {
                    anyhow::bail!("The `wamr` engine is not enabled in this build.")
                }
            }
            RuntimeType::Wasmi => {
                #[cfg(feature = "wasmi")]
                return Ok(wasmer::wasmi::Wasmi::new().into());
                #[allow(unreachable_code)]
                {
                    anyhow::bail!("The `wasmi` engine is not enabled in this build.")
                }
            }
            #[cfg(feature = "compiler")]
            _ => self.get_compiler_engine_for_target(target),

            #[cfg(not(feature = "compiler"))]
            _ => anyhow::bail!("No engine selected!"),
        }
    }

    #[cfg(feature = "compiler")]
    pub fn get_compiler_engine_for_target(
        &self,
        target: Target,
    ) -> std::result::Result<Engine, anyhow::Error> {
        let rt = self.get_rt()?;
        let compiler_config = self.get_compiler_config(&rt)?;
        let features = self.get_features(compiler_config.default_features_for_target(&target))?;
        Ok(wasmer_compiler::EngineBuilder::new(compiler_config)
            .set_features(Some(features))
            .set_target(Some(target))
            .engine()
            .into())
    }

    #[allow(unused_variables)]
    #[cfg(feature = "compiler")]
    pub(crate) fn get_compiler_config(&self, rt: &RuntimeType) -> Result<Box<dyn CompilerConfig>> {
        let compiler_config: Box<dyn CompilerConfig> = match rt {
            RuntimeType::Headless => bail!("The headless engine can't be chosen"),
            #[cfg(feature = "singlepass")]
            RuntimeType::Singlepass => {
                let mut config = wasmer_compiler_singlepass::Singlepass::new();
                if self.enable_verifier {
                    config.enable_verifier();
                }
                Box::new(config)
            }
            #[cfg(feature = "cranelift")]
            RuntimeType::Cranelift => {
                let mut config = wasmer_compiler_cranelift::Cranelift::new();
                if self.enable_verifier {
                    config.enable_verifier();
                }
                Box::new(config)
            }
            #[cfg(feature = "llvm")]
            RuntimeType::LLVM => {
                use std::{fmt, fs::File, io::Write};

                use wasmer_compiler_llvm::{
                    CompiledKind, InkwellMemoryBuffer, InkwellModule, LLVMCallbacks, LLVM,
                };
                use wasmer_types::entity::EntityRef;
                let mut config = LLVM::new();
                struct Callbacks {
                    debug_dir: PathBuf,
                }
                impl Callbacks {
                    fn new(debug_dir: PathBuf) -> Result<Self> {
                        // Create the debug dir in case it doesn't exist
                        std::fs::create_dir_all(&debug_dir)?;
                        Ok(Self { debug_dir })
                    }
                }
                // Converts a kind into a filename, that we will use to dump
                // the contents of the IR object file to.
                fn types_to_signature(types: &[Type]) -> String {
                    types
                        .iter()
                        .map(|ty| match ty {
                            Type::I32 => "i".to_string(),
                            Type::I64 => "I".to_string(),
                            Type::F32 => "f".to_string(),
                            Type::F64 => "F".to_string(),
                            Type::V128 => "v".to_string(),
                            Type::ExternRef => "e".to_string(),
                            Type::FuncRef => "r".to_string(),
                            Type::ExceptionRef => "x".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("")
                }
                // Converts a kind into a filename, that we will use to dump
                // the contents of the IR object file to.
                fn function_kind_to_filename(kind: &CompiledKind) -> String {
                    match kind {
                        CompiledKind::Local(local_index) => {
                            format!("function_{}", local_index.index())
                        }
                        CompiledKind::FunctionCallTrampoline(func_type) => format!(
                            "trampoline_call_{}_{}",
                            types_to_signature(func_type.params()),
                            types_to_signature(func_type.results())
                        ),
                        CompiledKind::DynamicFunctionTrampoline(func_type) => format!(
                            "trampoline_dynamic_{}_{}",
                            types_to_signature(func_type.params()),
                            types_to_signature(func_type.results())
                        ),
                        CompiledKind::Module => "module".into(),
                    }
                }
                impl LLVMCallbacks for Callbacks {
                    fn preopt_ir(&self, kind: &CompiledKind, module: &InkwellModule) {
                        let mut path = self.debug_dir.clone();
                        path.push(format!("{}.preopt.ll", function_kind_to_filename(kind)));
                        module
                            .print_to_file(&path)
                            .expect("Error while dumping pre optimized LLVM IR");
                    }
                    fn postopt_ir(&self, kind: &CompiledKind, module: &InkwellModule) {
                        let mut path = self.debug_dir.clone();
                        path.push(format!("{}.postopt.ll", function_kind_to_filename(kind)));
                        module
                            .print_to_file(&path)
                            .expect("Error while dumping post optimized LLVM IR");
                    }
                    fn obj_memory_buffer(
                        &self,
                        kind: &CompiledKind,
                        memory_buffer: &InkwellMemoryBuffer,
                    ) {
                        let mut path = self.debug_dir.clone();
                        path.push(format!("{}.o", function_kind_to_filename(kind)));
                        let mem_buf_slice = memory_buffer.as_slice();
                        let mut file = File::create(path)
                            .expect("Error while creating debug object file from LLVM IR");
                        let mut pos = 0;
                        while pos < mem_buf_slice.len() {
                            pos += file.write(&mem_buf_slice[pos..]).unwrap();
                        }
                    }
                }

                impl fmt::Debug for Callbacks {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "LLVMCallbacks")
                    }
                }

                if let Some(ref llvm_debug_dir) = self.llvm_debug_dir {
                    config.callbacks(Some(Arc::new(Callbacks::new(llvm_debug_dir.clone())?)));
                }
                if self.enable_verifier {
                    config.enable_verifier();
                }
                Box::new(config)
            }
            RuntimeType::V8 | RuntimeType::Wamr | RuntimeType::Wasmi => unreachable!(),
            #[cfg(not(all(feature = "singlepass", feature = "cranelift", feature = "llvm")))]
            compiler => {
                bail!(
                    "The `{}` compiler is not included in this binary.",
                    compiler.to_string()
                )
            }
        };

        #[allow(unreachable_code)]
        Ok(compiler_config)
    }
}

/// The compiler used for the store
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, dead_code)]
pub enum RuntimeType {
    /// Singlepass compiler
    Singlepass,

    /// Cranelift compiler
    Cranelift,

    /// LLVM compiler
    LLVM,

    /// V8 runtime
    V8,

    /// Wamr runtime
    Wamr,

    /// Wasmi runtime
    Wasmi,

    /// Headless compiler
    #[allow(dead_code)]
    Headless,
}

impl RuntimeType {
    /// Return all enabled compilers
    #[allow(dead_code)]
    pub fn enabled() -> Vec<RuntimeType> {
        vec![
            #[cfg(feature = "singlepass")]
            Self::Singlepass,
            #[cfg(feature = "cranelift")]
            Self::Cranelift,
            #[cfg(feature = "llvm")]
            Self::LLVM,
            #[cfg(feature = "v8")]
            Self::V8,
            #[cfg(feature = "wamr")]
            Self::Wamr,
        ]
    }
}

impl std::fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Singlepass => "singlepass",
                Self::Cranelift => "cranelift",
                Self::LLVM => "llvm",
                Self::V8 => "v8",
                Self::Wamr => "wamr",
                Self::Wasmi => "wasmi",
                Self::Headless => "headless",
            }
        )
    }
}
