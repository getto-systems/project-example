import { render, h } from "preact"

import { SignEntry } from "../../../../../auth/sign/sign/x_preact/sign"
import { ApplicationErrorComponent } from "../../../../../avail/x_preact/application_error"

import { newSignViewWorkerForeground } from "../../../../../auth/sign/sign/init/worker/foreground"
import { newWorkerForegroundOutsideFeature } from "../../../../../x_outside_feature/worker"

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
