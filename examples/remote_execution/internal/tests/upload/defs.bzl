# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

def _tests(ctx):
    # Create locally
    stage0 = ctx.actions.declare_output("stage0")
    ctx.actions.run(
        ["sh", "-c", 'head -c 1234 /dev/urandom > "$1"', "--", stage0.as_output()],
        category = "stage0",
        local_only = True,
    )

    # Use on RE, twice
    stage1 = ctx.actions.declare_output("stage1")
    ctx.actions.run(["sh", "-c", 'cat "$1" "$1" > "$2"', "--", stage0, stage1.as_output()], category = "stage1")

    # Depends on stage0 and stage1. Expect no uploads.
    stage2 = ctx.actions.declare_output("stage2")
    ctx.actions.run(
        ["sh", "-c", 'cat "$1" "$1" > "$2"', "--", stage0, stage2.as_output()],
        category = "stage2",
        env = {"__UNUSED": stage1},
    )

    return [DefaultInfo(stage2)]

tests = rule(attrs = {}, impl = _tests)
