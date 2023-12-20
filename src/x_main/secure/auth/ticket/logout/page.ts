import { h } from "preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { useNotifyUnexpectedError } from "../../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../common/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../z_vendor/getto-css/preact/layout/app"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { DisplayOutlineMenu } from "../../../../../common/outline/load/x_preact/display_menu"
import { DisplayOutlineBreadcrumbList } from "../../../../../common/outline/load/x_preact/display_breadcrumb_list"
import { Logout } from "../../../../../auth/ticket/logout/x_preact/logout"

import { LogoutPageResource } from "./resource"

export function LogoutPage(props: LogoutPageResource): PreactNode {
    const pageTitle = "ログアウト" as const

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(DisplayOutlineBreadcrumbList, props)]),
            body: mainBody(h(Logout, props)),
            copyright,
        }),
        menu: h(DisplayOutlineMenu, props),
    })
}
