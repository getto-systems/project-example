import { h, VNode } from "preact"

import { useApplicationView } from "../../../z_vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../z_vendor/getto-css/preact/layout/app"
import { box_double, container } from "../../../z_vendor/getto-css/preact/design/box"

import { copyright, siteInfo } from "../../../x_content/site"

import { ApplicationError } from "../../../avail/x_preact/application_error"
import { LoadSeason } from "../../season/load/x_preact/load_season"
import { LoadMenu } from "../../outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../outline/load/x_preact/load_breadcrumb_list"
import { LoadSeasonField } from "../../season/load/x_preact/load_season_field"

import { DashboardResource } from "../resource"
import { ApplicationView } from "../../../z_vendor/getto-application/action/action"

export function Dashboard(view: ApplicationView<DashboardResource>): VNode {
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
            body: mainBody(h(Example, resource)),
            copyright,
        }),
        menu: h(LoadMenu, resource),
    })
}

function Example(resource: DashboardResource): VNode {
    return container([
        box_double({
            title: "GETTO Example",
            body: h(LoadSeasonField, resource),
        }),
    ])
}
