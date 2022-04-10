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

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { LoadSeason } from "../../../../../core/season/load/x_preact/load_season"
import { LoadMenu } from "../../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { SearchAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/search"
import { ListAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/list"
import { DetailAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/detail"
import { MainTitleWithSidebar } from "../../../../../z_lib/ui/search/sidebar/x_preact/main_title"

import { isSidebarExpand } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(view: ApplicationView<ManageUserAccountPageResource>): VNode {
    const pageTitle = "ユーザー"
    const detailTitle = "ユーザー詳細"
    const listTitle = "一覧"

    useDocumentTitle(pageTitle)
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    const sidebarState = useApplicationAction(resource.sidebar)
    const detailState = useApplicationAction(resource.search.detail)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    const common = {
        siteInfo,
        header: [h(LoadSeason, resource)],
        menu: h(LoadMenu, resource),
    }

    switch (detailState.type) {
        case "initial":
            return appLayout({
                ...common,
                main: appMain({
                    header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, resource)]),
                    body: mainBody(h(SearchAuthUserAccount, resource)),
                    copyright,
                }),
            })

        case "focus-failed":
        case "focus-detected":
        case "focus-on":
            return appLayout({
                ...common,
                main: appMain({
                    header: mainHeader([
                        h(MainTitleWithSidebar, {
                            sidebar: resource.sidebar,
                            title: detailTitle,
                        }),
                        h(LoadBreadcrumbList, resource),
                    ]),
                    body: mainBody(
                        h(DetailAuthUserAccount, {
                            ...resource,
                            user:
                                detailState.type === "focus-failed"
                                    ? { found: false }
                                    : { found: true, user: detailState.user },
                        }),
                    ),
                    copyright,
                }),
                sidebar: isSidebarExpand(sidebarState)
                    ? appSidebar({
                          header: mainHeader([mainTitle(listTitle)]),
                          body: sidebarBody(h(ListAuthUserAccount, resource), {
                              id: "sidebar",
                          }),
                          copyright,
                      })
                    : undefined,
            })
    }
}
