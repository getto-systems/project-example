import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../ui/vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../../src/avail/unexpected_error/_ui/action_notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../src/example/_ui/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../src/example/site"

import { ApplicationErrorComponent } from "../../../../../src/avail/_ui/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../src/example/_ui/common/action_load_season/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../src/example/outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../src/example/outline/_ui/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { LogoutEntry } from "../../../../../src/auth/auth_ticket/_ui/action_logout/x_preact/logout"

import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export function ProfileEntry(view: ApplicationView<ProfilePageResource>): VNode {
    const resource = useApplicationView(view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(ProfileComponent, resource)
}

const pageTitle = "プロフィール" as const

export function ProfileComponent(props: ProfilePageResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, props)]),
            body: mainBody(h(LogoutEntry, props)),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
