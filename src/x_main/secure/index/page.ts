import { h, VNode } from "preact"

import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../common/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../z_vendor/getto-css/preact/layout/app"

import { copyright, siteInfo } from "../../../x_content/site"

import { ApplicationError } from "../../../avail/x_preact/application_error"
import { DisplaySeason } from "../../../core/season/load/x_preact/display_season"
import { DisplayOutlineBreadcrumbList } from "../../../common/outline/load/x_preact/display_breadcrumb_list"
import { DisplayOutlineMenu } from "../../../common/outline/load/x_preact/display_menu"
import { Dashboard } from "../../../x_content/x_preact/dashboard"

import { BaseResource } from "../base/resource"

export function DashboardPage(props: BaseResource): VNode {
    const pageTitle = "ホーム"

    useDocumentTitle(pageTitle)

    const err = useNotifyUnexpectedError(props)
    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(DisplaySeason, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(DisplayOutlineBreadcrumbList, props)]),
            body: mainBody(h(Dashboard, props)),
            copyright,
        }),
        menu: h(DisplayOutlineMenu, props),
    })
}
