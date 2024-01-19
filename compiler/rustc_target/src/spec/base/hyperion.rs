use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, StackProbeType, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "hyperion".into(),
        has_thread_local: true,
        // linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        panic_strategy: PanicStrategy::Abort,
        // singlethread: true,
        stack_probes: StackProbeType::Inline,
        tls_model: TlsModel::InitialExec,

        ..Default::default()
    }
}
