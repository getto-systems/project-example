import { render, h } from "preact"

import { SignEntry } from "../../../../../../src/auth/sign/action_sign/x_preact/sign"
import { ApplicationErrorComponent } from "../../../../../../src/avail/x_preact/application_error"

import { newSignViewWorkerForeground } from "../../../../../../src/auth/sign/action_sign/init/worker/foreground"
import { newWorkerForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/worker"

renderEntry()

async function renderEntry() {
    try {
        render(
            h(SignEntry, newSignViewWorkerForeground(await newWorkerForegroundOutsideFeature())),
            document.body,
        )
    } catch (err) {
        render(h(ApplicationErrorComponent, { err: `${err}` }), document.body)
    }
}
