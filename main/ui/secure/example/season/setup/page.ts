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
import { SetupSeasonEntry } from "../../../../../../src/example/season/setup/x_preact/setup"

import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"
import { SetupSeasonPageResource } from "./resource"
import { container } from "../../../../../../ui/vendor/getto-css/preact/design/box"

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
