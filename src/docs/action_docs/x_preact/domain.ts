import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationView } from "../../../../ui/vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/unexpected_error/notify/x_preact/hooks"
import { useDocumentTitle } from "../../../example/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../ui/vendor/getto-css/preact/layout/app"
import { container } from "../../../../ui/vendor/getto-css/preact/design/box"
import { v_small } from "../../../../ui/vendor/getto-css/preact/design/alignment"

import { copyright, siteInfo } from "../../../example/site"
import { dataBox, domainBox, usecaseAbstractBox } from "./helper"

import { ApplicationErrorComponent } from "../../../avail/x_preact/application_error"
import { LoadMenuEntry } from "../../../example/outline/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../example/outline/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"

import { DocsView, DocsResource } from "../resource"
import { DocsDomainContent } from "../../../../ui/vendor/getto-application/docs/data"

type EntryProps = Readonly<{
    view: DocsView
    docs: DocsDomainContent
}>
export function DocsDomainEntry(props: EntryProps): VNode {
    const resource = useApplicationView(props.view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DocsDomainComponent, { ...resource, docs: props.docs })
}

type Props = DocsResource & Readonly<{ docs: DocsDomainContent }>
export function DocsDomainComponent(resource: Props): VNode {
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
        return `${resource.docs.title}概要`
    }
}

function content(docs: DocsDomainContent): VNode {
    return html`${[
        container(domainBox(docs)),
        container(docs.usecase.map(usecaseAbstractBox)),
        v_small(),
        container(docs.data.map(dataBox)),
    ]}`
}
