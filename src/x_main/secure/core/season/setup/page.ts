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
import { useDocumentTitle } from "../../../../../example/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationErrorComponent } from "../../../../../avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../example/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../example/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../example/outline/load/x_preact/load_breadcrumb_list"
import { SetupSeasonEntry } from "../../../../../example/season/setup/x_preact/setup"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { SetupSeasonPageResource } from "./resource"
import { container } from "../../../../../z_vendor/getto-css/preact/design/box"

const pageTitle = "シーズン設定" as const

export function SetupSeasonPageEntry(view: ApplicationView<SetupSeasonPageResource>): VNode {
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(SetupSeasonPageComponent, resource)
}

export function SetupSeasonPageComponent(props: SetupSeasonPageResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, props)]),
            body: mainBody(container([h(SetupSeasonEntry, props)])),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
