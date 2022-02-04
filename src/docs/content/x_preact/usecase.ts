import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationView } from "../../../z_vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../example/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../z_vendor/getto-css/preact/layout/app"
import { container } from "../../../z_vendor/getto-css/preact/design/box"

import { copyright, siteInfo } from "../../../x_content/site"
import { docsActionBox, docsDataBox, docsUsecaseBox } from "./helper"

import { ApplicationErrorComponent } from "../../../avail/x_preact/application_error"
import { LoadMenuEntry } from "../../../example/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../example/outline/load_breadcrumb_list/x_preact/load_breadcrumb_list"

import { DocsView, DocsResource } from "../resource"
import { DocsData, DocsUsecase } from "../../../z_vendor/getto-application/docs/data"

type EntryProps = Readonly<{
    view: DocsView
    docs: DocsUsecase
}>
export function DocsUsecaseEntry(props: EntryProps): VNode {
    const resource = useApplicationView(props.view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DocsUsecaseComponent, { ...resource, docs: props.docs })
}

type Props = DocsResource & Readonly<{ docs: DocsUsecase }>
export function DocsUsecaseComponent(resource: Props): VNode {
    useDocumentTitle(title())

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle(title()), h(LoadBreadcrumbListComponent, resource)]),
            body: mainBody(content(resource.docs)),
            copyright,
        }),
        menu: h(LoadMenuEntry, resource),
    })

    function title() {
        return resource.docs.title
    }
}

function content(docs: DocsUsecase): VNode {
    return html`${[
        container(docsUsecaseBox(docs)),
        container(docs.action.map(docsActionBox)),
        container(data(docs).map(docsDataBox)),
    ]}`

    function data(usecase: DocsUsecase): readonly DocsData[] {
        return usecase.action
            .flatMap((action) => action.data)
            .reduce((acc, data) => {
                if (!acc.includes(data)) {
                    acc.push(data)
                }
                return acc
            }, <DocsData[]>[])
    }
}
