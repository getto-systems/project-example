import { h, VNode } from "preact"

import {
    useApplicationAction,
    useApplicationView,
} from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    appSidebar,
    mainBody,
    mainHeader,
    mainTitle,
    sidebarBody,
} from "../../../../../z_vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../../core/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationErrorComponent } from "../../../../../avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../../core/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { SearchAuthUserAccountEntry } from "../../../../../auth/user/account/search/x_preact/search"
import { ListAuthUserAccountEntry } from "../../../../../auth/user/account/search/x_preact/list"
import { DetailAuthUserAccountEntry } from "../../../../../auth/user/account/search/x_preact/detail"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPageEntry(
    view: ApplicationView<ManageUserAccountPageResource>,
): VNode {
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(ManageUserAccountPageComponent, resource)
}

const pageTitle = "ユーザー" as const
const detailTitle = "ユーザー詳細" as const
const listTitle = "一覧" as const

export function ManageUserAccountPageComponent(props: ManageUserAccountPageResource): VNode {
    useDocumentTitle(pageTitle)

    const detailState = useApplicationAction(props.search.detail)

    const common = {
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        menu: h(LoadMenuEntry, props),
    }

    switch (detailState.type) {
        case "initial-detail":
            return appLayout({
                ...common,
                main: appMain({
                    header: mainHeader([
                        mainTitle(pageTitle),
                        h(LoadBreadcrumbListComponent, props),
                    ]),
                    body: mainBody(h(SearchAuthUserAccountEntry, props)),
                    copyright,
                }),
            })

        case "focus-failed":
        case "focus-on":
            return appLayout({
                ...common,
                main: appMain({
                    header: mainHeader([mainTitle(detailTitle)]),
                    body: mainBody(
                        h(DetailAuthUserAccountEntry, {
                            detail: props.search.detail,
                            override: props.override,
                            user:
                                detailState.type === "focus-failed"
                                    ? { found: false }
                                    : { found: true, user: detailState.user },
                        }),
                    ),
                    copyright,
                }),
                sidebar: appSidebar({
                    header: mainHeader([mainTitle(listTitle)]),
                    body: sidebarBody(h(ListAuthUserAccountEntry, { list: props.search })),
                    copyright,
                }),
            })
    }
}
