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
import { useDocumentTitle } from "../../../../../common/x_preact/hooks"

import { copyright, siteInfo } from "../../../../../x_content/site"

import { ApplicationError } from "../../../../../avail/x_preact/application_error"
import { LoadSeason } from "../../../../../common/season/load/x_preact/load_season"
import { LoadMenu } from "../../../../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { SearchAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/search"
import { ListAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/list"
import { FocusedAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/focused"
import { MainTitleWithSidebar } from "../../../../../z_lib/ui/search/sidebar/x_preact/main_title"

import { isSidebarExpand } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { ApplicationView } from "../../../../../z_vendor/getto-application/action/action"
import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(view: ApplicationView<ManageUserAccountPageResource>): VNode {
    const pageTitle = "ユーザー"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "一覧"

    useDocumentTitle(pageTitle)
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    const sidebarState = useApplicationAction(resource.sidebar)
    const focusedState = useApplicationAction(resource.search.focused)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, resource)],
        menu: h(LoadMenu, resource),
        ...content(),
    })

    function content() {
        switch (focusedState.type) {
            case "initial":
                return {
                    main: appMain({
                        header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, resource)]),
                        body: mainBody(h(SearchAuthUserAccount, resource)),
                        copyright,
                    }),
                }

            case "focus-failed":
            case "focus-detected":
            case "focus-on":
                return {
                    main: appMain({
                        header: mainHeader([
                            h(MainTitleWithSidebar, {
                                sidebar: resource.sidebar,
                                title: focusedTitle,
                            }),
                            h(LoadBreadcrumbList, resource),
                        ]),
                        body: mainBody(
                            h(FocusedAuthUserAccount, {
                                ...resource,
                                focused: resource.search.focused,
                                user:
                                    focusedState.type === "focus-failed"
                                        ? { found: false }
                                        : { found: true, user: focusedState.user },
                            }),
                        ),
                        copyright,
                    }),
                    sidebar: isSidebarExpand(sidebarState)
                        ? appSidebar({
                              header: mainHeader([mainTitle(sidebarTitle)]),
                              body: sidebarBody(
                                  h(ListAuthUserAccount, { ...resource, list: resource.search }),
                                  { id: "sidebar" },
                              ),
                              copyright,
                          })
                        : undefined,
                }
        }
    }
}
