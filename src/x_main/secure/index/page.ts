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
import { LoadSeason } from "../../../core/season/load/x_preact/load_season"
import { LoadBreadcrumbList } from "../../../common/outline/load/x_preact/load_breadcrumb_list"
import { OutlineMenu } from "../../../common/outline/load/x_preact/outline_menu"
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
        header: [h(LoadSeason, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, props)]),
            body: mainBody(h(Dashboard, props)),
            copyright,
        }),
        menu: h(OutlineMenu, props),
    })
}
