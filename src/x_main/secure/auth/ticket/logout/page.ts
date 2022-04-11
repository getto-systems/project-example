import { h, VNode } from "preact"

import { useApplicationView } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../core/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../z_vendor/getto-css/preact/layout/app"
import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { LoadSeason } from "../../../../../core/season/load/x_preact/load_season"
import { LoadMenu } from "../../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { Logout } from "../../../../../auth/ticket/logout/x_preact/logout"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { LogoutPageResource } from "./resource"

export function LogoutPage(view: ApplicationView<LogoutPageResource>): VNode {
    const pageTitle = "ログアウト" as const

    useDocumentTitle(pageTitle)
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, resource)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, resource)]),
            body: mainBody(container(h(Logout, resource))),
            copyright,
        }),
        menu: h(LoadMenu, resource),
    })
}
