use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "x86_64-unknown-none".into(),
        pointer_width: 64,
        arch: "x86_64".into(),
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        options: TargetOptions {
            cpu: "x86-64".into(),
            features: "+rdrnd,+rdseed".into(),
            max_atomic_width: Some(64),
            ..base::hyperion::opts()
        },
    }
}
