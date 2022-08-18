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
import { OutlineMenu } from "../../../../../common/outline/load/x_preact/outline_menu"
import { LoadBreadcrumbList } from "../../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { MainTitleWithSidebar } from "../../../../../z_lib/ui/search/sidebar/x_preact/main_title"
import { RegisterAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/register"
import { ListRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/list"
import { FocusRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/focus"

import { isSidebarExpand } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { RegisterUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(props: RegisterUserAccountPageResource): VNode {
    const pageTitle = "ユーザー登録"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "登録済み一覧"

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    const sidebarState = useApplicationState(props.sidebar.state)
    const focusState = useApplicationState(props.register.list.focus.state)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        menu: h(OutlineMenu, props),
        main:
            focusState.type === "close"
                ? appMain({
                      header: mainHeader([
                          h(MainTitleWithSidebar, {
                              sidebar: props.sidebar,
                              title: pageTitle,
                          }),
                          h(LoadBreadcrumbList, props),
                      ]),
                      body: mainBody(h(RegisterAuthUserAccount, props)),
                      copyright,
                  })
                : appMain({
                      header: mainHeader([
                          h(MainTitleWithSidebar, {
                              sidebar: props.sidebar,
                              title: focusedTitle,
                          }),
                          h(LoadBreadcrumbList, props),
                      ]),
                      body: mainBody(h(FocusRegisteredAuthUserAccount, props)),
                      copyright,
                  }),
        sidebar: isSidebarExpand(sidebarState)
            ? appSidebar({
                  header: mainHeader([mainTitle(sidebarTitle)]),
                  body: sidebarBody(h(ListRegisteredAuthUserAccount, props), { id: "sidebar" }),
                  copyright,
              })
            : undefined,
    })
}
