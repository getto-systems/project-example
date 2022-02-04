import { render, h } from "preact"

import { newSignView } from "../../../../auth/sign/sign/init/resource"

import { SignEntry } from "../../../../auth/sign/sign/x_preact/sign"
import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

render(
    h(SignEntry, newSignView(newForegroundOutsideFeature())),
    document.body,
)
