import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../ui/vendor/getto-css/preact/layout/app"
import { container } from "../../../../../ui/vendor/getto-css/preact/design/box"

import { useNotifyUnexpectedError } from "../../../../../src/avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../src/example/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../src/x_content/site"

import { ApplicationErrorComponent } from "../../../../../src/avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../src/example/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../src/example/outline/load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../src/example/outline/load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { ChangePasswordEntry } from "../../../../../src/auth/user/password/change/x_preact/change_password"
import { RequestResetTokenProfileEntry } from "../../../../../src/auth/user/password/reset/request_token/x_preact/request_token_profile"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export function ProfilePageEntry(view: ApplicationView<ProfilePageResource>): VNode {
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(ProfilePageComponent, resource)
}

const pageTitle = "プロフィール" as const

export function ProfilePageComponent(props: ProfilePageResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, props)]),
            body: mainBody(
                container([h(ChangePasswordEntry, props), h(RequestResetTokenProfileEntry, props)]),
            ),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
