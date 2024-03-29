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
import { MainTitleWithSidebar } from "../../../../../common/util/sidebar/x_preact/main_title"
import { RegisterAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/register"
import { ListRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/list"
import { FocusRegisteredAuthUserAccount } from "../../../../../auth/user/account/register/x_preact/focus"

import { isSidebarExpand } from "../../../../../common/util/sidebar/x_preact/helper"

import { RegisterUserAccountPageResource } from "./resource"

export function RegisterUserAccountPage(props: RegisterUserAccountPageResource): PreactNode {
    const pageTitle = "ユーザー登録"
    const focusedTitle = "ユーザー詳細"
    const sidebarTitle = "登録済み一覧"

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    const sidebarState = useAtom(props.sidebar.state)
    const focusState = useAtom(props.register.focus.detect)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        menu: h(DisplayOutlineMenu, props),
        main: focusState.found
            ? appMain({
                  header: mainHeader([
                      h(MainTitleWithSidebar, {
                          sidebar: props.sidebar,
                          title: focusedTitle,
                      }),
                      h(DisplayOutlineBreadcrumbList, props),
                  ]),
                  body: mainBody(h(FocusRegisteredAuthUserAccount, props)),
                  copyright,
              })
            : appMain({
                  header: mainHeader([
                      h(MainTitleWithSidebar, {
                          sidebar: props.sidebar,
                          title: pageTitle,
                      }),
                      h(DisplayOutlineBreadcrumbList, props),
                  ]),
                  body: mainBody(h(RegisterAuthUserAccount, props)),
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
