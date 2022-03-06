import { render, h } from "preact"
import { initSignLink } from "../../../../auth/sign/nav/action"

import { newSignView } from "../../../../auth/sign/sign/init/resource"

import { SignEntry } from "../../../../auth/sign/sign/x_preact/sign"
import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

render(
    h(SignEntry, {
        link: initSignLink(),
        sign: newSignView(newForegroundOutsideFeature()),
    }),
    document.body,
)
