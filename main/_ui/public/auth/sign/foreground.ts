import { render, h } from "preact"

import { newSignViewWorkerForeground } from "../../../../../src/auth/sign/action_sign/init/worker/foreground"

import { SignEntry } from "../../../../../src/auth/sign/action_sign/x_preact/sign"
import { newWorkerForegroundOutsideFeature } from "../../../../../src/x_outside_feature/worker"

render(
    h(SignEntry, newSignViewWorkerForeground(newWorkerForegroundOutsideFeature())),
    document.body,
)
