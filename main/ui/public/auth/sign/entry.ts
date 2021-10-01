import { render, h } from "preact"

import { newSignView } from "../../../../../src/auth/sign/action_sign/init/resource"

import { SignEntry } from "../../../../../src/auth/sign/action_sign/x_preact/sign"
import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/common"

render(
    h(SignEntry, newSignView(newForegroundOutsideFeature())),
    document.body,
)