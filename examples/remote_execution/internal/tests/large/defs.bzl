# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

def _tests(ctx):
    # Create a large blob locally
    stage0 = ctx.actions.declare_output("stage0")
    ctx.actions.run(
        ["sh", "-c", 'head -c 8388608 /dev/urandom > "$1"', "--", stage0.as_output()],
        category = "stage0",
        local_only = True,
    )

    # Use on RE
    stage1 = ctx.actions.declare_output("stage1")
    ctx.actions.run(["cp", stage0, stage1.as_output()], category = "stage1")

    # Download it back
    stage2 = ctx.actions.declare_output("stage2")
    ctx.actions.run(["cp", stage1, stage2.as_output()], category = "stage2")

    # Verify
    stage3 = ctx.actions.declare_output("stage3")
    ctx.actions.run(
        [
            "sh",
            "-c",
            'diff "$1" "$2" && touch "$3"',
            "--",
            stage0,
            stage2,
            stage3.as_output(),
        ],
        category = "stage3",
    )

    return [DefaultInfo(stage3)]

tests = rule(attrs = {}, impl = _tests)
