import { h, render } from "preact"

import { ProfilePage } from "./page"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newBaseResource } from "../../base/init/resource"
import { newChangePasswordAction } from "../../../../auth/user/password/change/init/resource"
import { newRequestResetTokenAction } from "../../../../auth/user/password/reset/request_token/init/resource"

import { ProfilePageResource } from "./resource"

render(h(ProfilePage, props()), document.body)

function props(): ProfilePageResource {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        change: newChangePasswordAction(feature),
        requestToken: newRequestResetTokenAction(feature),
    }
}
