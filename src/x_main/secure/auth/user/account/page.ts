import { h } from "preact"
import { PreactNode } from "../../../../../common/x_preact/node"

import { useAtom } from "../../../../../z_vendor/getto-atom/x_preact/hooks"

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
import { DisplayOutlineMenu } from "../../../../../common/outline/load/x_preact/display_menu"
import { DisplayOutlineBreadcrumbList } from "../../../../../common/outline/load/x_preact/display_breadcrumb_list"
import { SearchAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/search"
import { ListAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/list"
import { FocusAuthUserAccount } from "../../../../../auth/user/account/search/x_preact/focus"
import { MainTitleWithSidebar } from "../../../../../common/util/sidebar/x_preact/main_title"

import { isSidebarExpand } from "../../../../../common/util/sidebar/x_preact/helper"

import { ManageUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(props: ManageUserAccountPageResource): PreactNode {
    const pageTitle = "ユーザー"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "一覧"

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    const sidebarState = useAtom(props.sidebar.state)
    const isFocused = useAtom(props.search.focus.isSomeEntryFocused)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        menu: h(DisplayOutlineMenu, props),
        ...(isFocused
            ? {
                  main: appMain({
                      header: mainHeader([
                          h(MainTitleWithSidebar, {
                              sidebar: props.sidebar,
                              title: focusedTitle,
                          }),
                          h(DisplayOutlineBreadcrumbList, props),
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
              }
            : {
                  main: appMain({
                      header: mainHeader([
                          mainTitle(pageTitle),
                          h(DisplayOutlineBreadcrumbList, props),
                      ]),
                      body: mainBody(h(SearchAuthUserAccount, props)),
                      copyright,
                  }),
              }),
    })
}
