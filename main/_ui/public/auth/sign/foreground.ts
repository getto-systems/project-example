import { render, h } from "preact"

import { newSignWorkerForeground } from "../../../../../src/auth/_ui/action_sign/init/worker/foreground"

import { SignEntry } from "../../../../../src/auth/_ui/action_sign/x_preact/sign"
import { workerForegroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/worker"

render(h(SignEntry, newSignWorkerForeground(workerForegroundOutsideFeature())), document.body)
