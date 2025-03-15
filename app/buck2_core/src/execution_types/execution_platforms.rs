/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::sync::Arc;

use allocative::Allocative;

use crate::execution_types::execution::ExecutionPlatform;
use crate::target::label::label::TargetLabel;

pub type ExecutionPlatforms = Arc<ExecutionPlatformsData>;

#[derive(Debug, Allocative)]
pub enum ExecutionPlatformFallback {
    Error,
    UseUnspecifiedExec,
    Platform(ExecutionPlatform),
}

#[derive(Debug, Allocative)]
pub struct ExecutionPlatformsData {
    execution_platforms_target: TargetLabel,
    platforms: Vec<ExecutionPlatform>,
    fallback: ExecutionPlatformFallback,
}

impl ExecutionPlatformsData {
    pub fn new(
        execution_platforms_target: TargetLabel,
        platforms: Vec<ExecutionPlatform>,
        fallback: ExecutionPlatformFallback,
    ) -> Self {
        Self {
            execution_platforms_target,
            platforms,
            fallback,
        }
    }

    pub fn execution_platforms_target(&self) -> &TargetLabel {
        &self.execution_platforms_target
    }

    pub fn candidates(&self) -> impl Iterator<Item = &ExecutionPlatform> {
        self.platforms.iter()
    }

    pub fn fallback(&self) -> &ExecutionPlatformFallback {
        &self.fallback
    }
}
