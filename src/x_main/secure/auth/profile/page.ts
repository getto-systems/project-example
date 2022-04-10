import { h, VNode } from "preact"

import { useApplicationView } from "../../../../z_vendor/getto-application/action/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../z_vendor/getto-css/preact/layout/app"
import { container } from "../../../../z_vendor/getto-css/preact/design/box"

import { useNotifyUnexpectedError } from "../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../core/x_preact/hooks"

import { copyright, siteInfo } from "../../../../x_content/site"

import { ApplicationError } from "../../../../avail/x_preact/application_error"
import { LoadSeason } from "../../../../core/season/load/x_preact/load_season"
import { LoadMenu } from "../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { ChangePassword } from "../../../../auth/user/password/change/x_preact/change_password"
import { RequestResetTokenProfile } from "../../../../auth/user/password/reset/request_token/x_preact/request_token_profile"

import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export function ProfilePage(view: ApplicationView<ProfilePageResource>): VNode {
    const pageTitle = "プロフィール" as const

    useDocumentTitle(pageTitle)
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(LoadSeason, resource)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbList, resource)]),
            body: mainBody(
                container([
                    h(ChangePassword, resource.change),
                    h(RequestResetTokenProfile, resource.requestToken),
                ]),
            ),
            copyright,
        }),
        menu: h(LoadMenu, resource),
    })
}
