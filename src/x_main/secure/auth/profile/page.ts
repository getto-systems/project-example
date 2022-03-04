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

import { ApplicationErrorComponent } from "../../../../avail/x_preact/application_error"
import { LoadSeasonEntry } from "../../../../core/season/load/x_preact/load_season"
import { LoadMenuEntry } from "../../../../core/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../../core/outline/load/x_preact/load_breadcrumb_list"
import { ChangePasswordEntry } from "../../../../auth/user/password/change/x_preact/change_password"
import { RequestResetTokenProfileEntry } from "../../../../auth/user/password/reset/request_token/x_preact/request_token_profile"

import { ApplicationView } from "../../../../z_vendor/getto-application/action/action"
import { ProfilePageResource } from "./resource"

export function ProfilePageEntry(view: ApplicationView<ProfilePageResource>): VNode {
    const resource = useApplicationView(view)
    const err = useNotifyUnexpectedError(resource)

    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }
    return h(ProfilePageComponent, resource)
}

const pageTitle = "プロフィール" as const

export function ProfilePageComponent(props: ProfilePageResource): VNode {
    useDocumentTitle(pageTitle)

    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(LoadBreadcrumbListComponent, props)]),
            body: mainBody(
                container([
                    h(ChangePasswordEntry, props),
                    h(RequestResetTokenProfileEntry, props.requestToken),
                ]),
            ),
            copyright,
        }),
        menu: h(LoadMenuEntry, props),
    })
}
