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

import { useNotifyUnexpectedError } from "../../../../../src/avail/unexpected_error/_ui/action_notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../src/example/_ui/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../src/example/site"

import { ApplicationErrorComponent } from "../../../../../src/avail/_ui/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../src/example/_ui/common/action_load_season/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../src/example/outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../src/example/outline/_ui/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { ChangePasswordEntry } from "../../../../../src/auth/user/password/action_change/x_preact/change_password"

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
            body: mainBody(container([h(ChangePasswordEntry, props)])),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
