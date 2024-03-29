import { PreactNode } from "../../common/x_preact/node"
import { h } from "preact"
import { html } from "htm/preact"

import { useNotifyUnexpectedError } from "../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../common/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../z_vendor/getto-css/preact/layout/app"
import { box, container } from "../../z_vendor/getto-css/preact/design/box"
import { field } from "../../z_vendor/getto-css/preact/design/form"

import { copyright, siteInfo } from "../../x_content/site"

import { ApplicationError } from "../../avail/x_preact/application_error"
import { DisplayOutlineMenu } from "../../common/outline/load/x_preact/display_menu"
import { DisplayOutlineBreadcrumbList } from "../../common/outline/load/x_preact/display_breadcrumb_list"

import { DocsResource } from "../resource"

import { DocsDescription } from "../../common/util/docs/data"

type Props = DocsResource &
    Readonly<{
        title: string
        docs: readonly DocsDescription[]
    }>
export function Docs(props: Props): PreactNode {
    useDocumentTitle(props.title)
    const err = useNotifyUnexpectedError(props)
    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle(props.title), h(DisplayOutlineBreadcrumbList, props)]),
            body: mainBody(content(props.docs)),
            copyright,
        }),
        menu: h(DisplayOutlineMenu, props),
    })
}

function content(docs: readonly DocsDescription[]): PreactNode {
    return container(
        docs.map((docs) =>
            box({
                title: docs.title,
                body: docs.descriptions.map((description) =>
                    field({
                        title: description.title,
                        body: html`<ul>
                            ${description.description.map(
                                (description) => html`<li>${description}</li>`,
                            )}
                        </ul>`,
                    }),
                ),
            }),
        ),
    )
}
