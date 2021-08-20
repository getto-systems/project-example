import { h, VNode } from "preact"

import { useApplicationView } from "../../../../ui/vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/_ui/action_notify/x_preact/hooks"
import { useDocumentTitle } from "../../../example/_ui/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../ui/vendor/getto-css/preact/layout/app"

import { copyright, siteInfo } from "../../../example/site"

import { ApplicationErrorComponent } from "../../../avail/_ui/x_preact/application_error"
import { LoadMenuEntry } from "../../../outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../outline/_ui/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"
import { docsArticle } from "./content"

import { DocsView, DocsResource } from "../resource"
import { DocsSection } from "../../../../ui/vendor/getto-application/docs/data"

export type DocsContent = Readonly<{
    title: string
    contents: DocsSection[][][]
}>

type EntryProps = Readonly<{
    view: DocsView
    docs: DocsContent
}>
export function DocsEntry(props: EntryProps): VNode {
    const resource = useApplicationView(props.view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DocsComponent, { ...resource, docs: props.docs })
}

type Props = DocsResource & Readonly<{ docs: DocsContent }>
export function DocsComponent(resource: Props): VNode {
    useDocumentTitle(resource.docs.title)

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([
                mainTitle(resource.docs.title),
                h(LoadBreadcrumbListComponent, resource),
            ]),
            body: mainBody(docsArticle(resource.docs.contents)),
            copyright,
        }),
        menu: h(LoadMenuEntry, resource),
    })
}
