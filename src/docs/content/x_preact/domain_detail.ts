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
import { v_small } from "../../../z_vendor/getto-css/preact/design/alignment"

import { copyright, siteInfo } from "../../../x_content/site"
import { docsActionBox, docsDataBox, docsDomainBox } from "./helper"

import { ApplicationErrorComponent } from "../../../avail/x_preact/application_error"
import { LoadMenuEntry } from "../../../example/outline/load/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../example/outline/load/x_preact/load_breadcrumb_list"

import { DocsView, DocsResource } from "../resource"
import { DocsData, DocsDomain } from "../../../z_vendor/getto-application/docs/data"

type EntryProps = Readonly<{
    view: DocsView
    docs: DocsDomain
}>
export function DocsDomainDetailEntry(props: EntryProps): VNode {
    const resource = useApplicationView(props.view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DocsDomainDetailComponent, { ...resource, docs: props.docs })
}

type Props = DocsResource & Readonly<{ docs: DocsDomain }>
export function DocsDomainDetailComponent(resource: Props): VNode {
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

function content(docs: DocsDomain): VNode {
    return html`${[
        container(docsDomainBox(docs)),
        container(docs.usecase.flatMap((usecase) => usecase.action.map(docsActionBox))),
        v_small(),
        container(data(docs).map(docsDataBox)),
    ]}`

    function data(docs: DocsDomain): readonly DocsData[] {
        return docs.usecase
            .flatMap((usecase) => usecase.action.flatMap((action) => action.data))
            .reduce((acc, purpose) => {
                if (!acc.includes(purpose)) {
                    acc.push(purpose)
                }
                return acc
            }, <DocsData[]>[])
    }
}
