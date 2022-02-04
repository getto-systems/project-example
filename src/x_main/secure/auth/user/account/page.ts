import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../z_vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../core/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationErrorComponent } from "../../../../../avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../core/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { ManageUserAccountEntry } from "../../../../../auth/user/account/manage/x_preact/manage"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPageEntry(
    view: ApplicationView<ManageUserAccountPageResource>,
): VNode {
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(ManageUserAccountPageComponent, resource)
}

const pageTitle = "ユーザー" as const

export function ManageUserAccountPageComponent(props: ManageUserAccountPageResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, props)]),
            body: mainBody(h(ManageUserAccountEntry, props)),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
