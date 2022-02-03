import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../../ui/vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../../ui/vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../../../src/avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../../src/example/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../../src/x_content/site"

import { ApplicationErrorComponent } from "../../../../../../src/avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../../src/example/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../../src/example/outline/load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../../src/example/outline/load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { ManageUserAccountEntry } from "../../../../../../src/auth/user/account/manage/x_preact/manage"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPageEntry(view: ApplicationView<ManageUserAccountPageResource>): VNode {
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
