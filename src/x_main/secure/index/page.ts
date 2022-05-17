import { h, VNode } from "preact"

import { useApplicationView } from "../../../z_vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../core/x_preact/hooks"

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
import { LoadBreadcrumbList } from "../../../core/outline/load/x_preact/load_breadcrumb_list"
import { LoadMenu } from "../../../core/outline/load/x_preact/load_menu"
import { Dashboard } from "../../../x_content/x_preact/dashboard"

import { ApplicationView } from "../../../z_vendor/getto-application/action/action"
import { BaseResource } from "../../../core/base/resource"

export function DashboardPage(view: ApplicationView<BaseResource>): VNode {
    const pageTitle = "ホーム"

    const resource = useApplicationView(view)

    useDocumentTitle(pageTitle)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, resource)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, resource)]),
            body: mainBody(h(Dashboard, resource)),
            copyright,
        }),
        menu: h(LoadMenu, resource),
    })
}
