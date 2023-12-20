import { PreactNode } from "../../../../common/x_preact/vnode"
import { h } from "preact"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../z_vendor/getto-css/preact/layout/app"

import { useNotifyUnexpectedError } from "../../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../../common/x_preact/hooks"

import { copyright, siteInfo } from "../../../../x_content/site"
import { container } from "../../../../z_vendor/getto-css/preact/design/box"

import { ApplicationError } from "../../../../avail/x_preact/application_error"
import { DisplaySeason } from "../../../../core/season/load/x_preact/display_season"
import { DisplayOutlineMenu } from "../../../../common/outline/load/x_preact/display_menu"
import { DisplayOutlineBreadcrumbList } from "../../../../common/outline/load/x_preact/display_breadcrumb_list"
import { SetupSeason } from "../../../../core/season/setup/x_preact/setup"

import { SetupSeasonPageResource } from "./resource"

export function SetupSeasonPage(props: SetupSeasonPageResource): PreactNode {
    const pageTitle = "シーズン設定" as const

    useDocumentTitle(pageTitle)
    const err = useNotifyUnexpectedError(props)

    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [h(DisplaySeason, props)],
        main: appMain({
            header: mainHeader([mainTitle(pageTitle), h(DisplayOutlineBreadcrumbList, props)]),
            body: mainBody(container([h(SetupSeason, props)])),
            copyright,
        }),
        menu: h(DisplayOutlineMenu, props),
    })
}
