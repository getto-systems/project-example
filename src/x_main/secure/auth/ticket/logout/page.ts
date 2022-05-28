import { h, VNode } from "preact"

import { useNotifyUnexpectedError } from "../../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../common/x_preact/hooks"

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
import { LoadMenu } from "../../../../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { Logout } from "../../../../../auth/ticket/logout/x_preact/logout"

import { LogoutPageResource } from "./resource"

export function LogoutPage(props: LogoutPageResource): VNode {
    const pageTitle = "ログアウト" as const

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, props)]),
            body: mainBody(container(h(Logout, props))),
            copyright,
        }),
        menu: h(LoadMenu, props),
    })
}
