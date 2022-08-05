import { h, VNode } from "preact"

import { useApplicationState } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

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
import { LoadMenu } from "../../../../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { SearchAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/search"
import { ListAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/list"
import { FocusAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/focus"
import { MainTitleWithSidebar } from "../../../../../z_lib/ui/search/sidebar/x_preact/main_title"

import { isSidebarExpand } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(props: ManageUserAccountPageResource): VNode {
    const pageTitle = "ユーザー"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "一覧"

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    const sidebarState = useApplicationState(props.sidebar.state)
    const focusState = useApplicationState(props.search.list.focus.state)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        menu: h(LoadMenu, props),
        ...(focusState.type === "close"
            ? {
                  main: appMain({
                      header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, props)]),
                      body: mainBody(h(SearchAuthUserAccount, props)),
                      copyright,
                  }),
              }
            : {
                  main: appMain({
                      header: mainHeader([
                          h(MainTitleWithSidebar, {
                              sidebar: props.sidebar,
                              title: focusedTitle,
                          }),
                          h(LoadBreadcrumbList, props),
                      ]),
                      body: mainBody(h(FocusAuthUserAccount, props)),
                      copyright,
                  }),
                  sidebar: isSidebarExpand(sidebarState)
                      ? appSidebar({
                            header: mainHeader([mainTitle(sidebarTitle)]),
                            body: sidebarBody(h(ListAuthUserAccount, props), { id: "sidebar" }),
                            copyright,
                        })
                      : undefined,
              }),
    })
}
