import { h, render } from "preact"

import { ProfilePage } from "./page"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newBaseResource } from "../../base/detail/resource"
import { newChangePasswordAction } from "../../../../auth/user/password/change/detail/resource"
import { newRequestResetTokenAction } from "../../../../auth/user/password/reset/request_token/detail/resource"

import { ProfilePageResource } from "./resource"

render(h(ProfilePage, props()), document.body)

function props(): ProfilePageResource {
    const feature = newForegroundOutsideFeature()
    return {
        ...newBaseResource(feature),
        change: newChangePasswordAction(),
        requestToken: newRequestResetTokenAction(),
    }
}
