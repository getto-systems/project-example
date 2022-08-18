import { h, VNode } from "preact"
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
import { OutlineMenu } from "../../common/outline/load/x_preact/load_menu"
import { LoadBreadcrumbList } from "../../common/outline/load/x_preact/load_breadcrumb_list"

import { DocsResource } from "../resource"

import { DocsDescription } from "../../z_vendor/getto-application/docs/data"

type Props = DocsResource &
    Readonly<{
        title: string
        docs: readonly DocsDescription[]
    }>
export function Docs(props: Props): VNode {
    useDocumentTitle(props.title)
    const err = useNotifyUnexpectedError(props)
    if (err) {
        return h(ApplicationError, { err: `${err}` })
    }

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle(props.title), h(LoadBreadcrumbList, props)]),
            body: mainBody(content(props.docs)),
            copyright,
        }),
        menu: h(OutlineMenu, props),
    })
}

function content(docs: readonly DocsDescription[]): VNode {
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
