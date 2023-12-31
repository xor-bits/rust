use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, StackProbeType, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "hyperion".into(),
        // linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        // tls_model: TlsModel::InitialExec,
        // has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}
