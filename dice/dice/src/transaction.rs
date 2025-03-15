/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use allocative::Allocative;
use dupe::Dupe;

use crate::impls::ctx::BaseComputeCtx;
use crate::versions::VersionNumber;
use crate::DiceComputations;
use crate::DiceTransactionUpdater;

#[derive(Allocative)]
pub(crate) enum DiceTransactionImpl {
    Modern(BaseComputeCtx),
}

impl Clone for DiceTransactionImpl {
    fn clone(&self) -> Self {
        match self {
            DiceTransactionImpl::Modern(ctx) => DiceTransactionImpl::Modern(ctx.clone()),
        }
    }
}

impl Dupe for DiceTransactionImpl {}

impl DiceTransactionImpl {
    pub(crate) fn get_version(&self) -> VersionNumber {
        match self {
            DiceTransactionImpl::Modern(ctx) => ctx.get_version(),
        }
    }

    pub(crate) fn into_updater(self) -> DiceTransactionUpdater {
        match self {
            DiceTransactionImpl::Modern(delegate) => delegate.into_updater(),
        }
    }

    pub(crate) fn as_computations(&self) -> &DiceComputations<'static> {
        match self {
            DiceTransactionImpl::Modern(ctx) => ctx.as_computations(),
        }
    }

    pub(crate) fn as_computations_mut(&mut self) -> &mut DiceComputations<'static> {
        match self {
            DiceTransactionImpl::Modern(ctx) => ctx.as_computations_mut(),
        }
    }
}
