import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../ui/vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../avail/unexpected_error/_ui/action_notify_unexpected_error/x_preact/hooks"
import { useDocumentTitle } from "../../../../example/_ui/x_preact/hooks"

import { copyright, siteInfo } from "../../../../example/site"

import { ApplicationErrorComponent } from "../../../../avail/_ui/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../example/_ui/common/action_load_season/x_preact/load_season"
import { LoadMenuEntry } from "../../../../outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../outline/_ui/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { LogoutEntry } from "../../../auth_ticket/_ui/action_logout/x_preact/logout"

import { ProfileView, ProfileResource } from "../resource"

export function ProfileEntry(view: ProfileView): VNode {
    const resource = useApplicationView(view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(ProfileComponent, resource)
}

const pageTitle = "プロフィール" as const

export function ProfileComponent(props: ProfileResource): VNode {
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
