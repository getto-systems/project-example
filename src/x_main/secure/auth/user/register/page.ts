import { h, VNode } from "preact"

import { useApplicationAction } from "../../../../../z_vendor/getto-application/action/x_preact/hooks"

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
import { LoadSeason } from "../../../../../core/season/load/x_preact/load_season"
import { LoadMenu } from "../../../../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { MainTitleWithSidebar } from "../../../../../z_lib/ui/search/sidebar/x_preact/main_title"
import { RegisterAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/register"
import { ListRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/list"
import { FocusedRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/focused"

import { isSidebarExpand } from "../../../../../z_lib/ui/search/sidebar/x_preact/helper"

import { RegisterUserAccountPageResource } from "./resource"

export function ManageUserAccountPage(props: RegisterUserAccountPageResource): VNode {
    const pageTitle = "ユーザー登録"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "登録済み一覧"

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    const sidebarState = useApplicationAction(props.sidebar)
    const focusedState = useApplicationAction(props.register.list.focused)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, props)],
        menu: h(LoadMenu, props),
        main:
            focusedState.type === "initial"
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
                      body: mainBody(
                          h(FocusedRegisteredAuthUserAccount, {
                              ...props,
                              focused: props.register.list.focused,
                              user: focusedState.user,
                          }),
                      ),
                      copyright,
                  }),
        sidebar: isSidebarExpand(sidebarState)
            ? appSidebar({
                  header: mainHeader([mainTitle(sidebarTitle)]),
                  body: sidebarBody(
                      h(ListRegisteredAuthUserAccount, {
                          ...props,
                          list: props.register.list,
                      }),
                      { id: "sidebar" },
                  ),
                  copyright,
              })
            : undefined,
    })
}
