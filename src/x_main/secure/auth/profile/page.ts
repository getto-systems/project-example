import { h, VNode } from "preact"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../z_vendor/getto-css/preact/layout/app"
import { container } from "../../../../z_vendor/getto-css/preact/design/box"

import { useNotifyUnexpectedError } from "../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../common/x_preact/hooks"

import { copyright, siteInfo } from "../../../../x_content/site"

import { ApplicationError } from "../../../../avail/x_preact/application_error"
import { LoadMenu } from "../../../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../common/outline/load/x_preact/load_breadcrumb_list"
import { ChangePassword } from "../../../../auth/user/password/change/x_preact/change_password"
import { RequestResetTokenProfile } from "../../../../auth/user/password/reset/request_token/x_preact/request_token_profile"

import { ProfilePageResource } from "./resource"

export function ProfilePage(props: ProfilePageResource): VNode {
    const pageTitle = "プロフィール" as const

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, props)]),
            body: mainBody(
                container([
                    h(ChangePassword, props),
                    h(RequestResetTokenProfile, props),
                ]),
            ),
            copyright,
        }),
        menu: h(LoadMenu, props),
    })
}
