import { h, VNode } from "preact"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../ui/vendor/getto-css/preact/layout/app"

import { useApplicationView } from "../../../../ui/vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../x_preact/hooks"

import { copyright, siteInfo } from "../../../x_content/site"

import { ApplicationErrorComponent } from "../../../avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../season/action_load/x_preact/load_season"
import { LoadMenuEntry } from "../../outline/load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../outline/load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { LoadSeasonFieldEntry } from "../../season/action_load/x_preact/load_season_field"

import { DashboardView, DashboardResource } from "../resource"
import { box_double, container } from "../../../../ui/vendor/getto-css/preact/design/box"

export function DashboardEntry(view: DashboardView): VNode {
    const resource = useApplicationView(view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DashboardComponent, resource)
}

const pageTitle = "ホーム" as const

export function DashboardComponent(resource: DashboardResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, resource)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, resource)]),
            body: mainBody(h(ExampleComponent, resource)),
            copyright,
        }),
        menu: h(LoadMenuEntry, resource),
    })
}

function ExampleComponent(resource: DashboardResource): VNode {
    return container([
        box_double({
            title: "GETTO Example",
            body: h(LoadSeasonFieldEntry, resource),
        }),
    ])
}
