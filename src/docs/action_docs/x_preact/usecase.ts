import { h, VNode } from "preact"
import { html } from "htm/preact"

import { useApplicationView } from "../../../../ui/vendor/getto-application/action/x_preact/hooks"
import { useNotifyUnexpectedError } from "../../../avail/_ui/action_notify_unexpected_error/x_preact/hooks"
import { useDocumentTitle } from "../../../example/_ui/x_preact/hooks"

import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../ui/vendor/getto-css/preact/layout/app"
import { container } from "../../../../ui/vendor/getto-css/preact/design/box"

import { copyright, siteInfo } from "../../../example/site"
import { actionBox, dataBox, usecaseBox } from "./helper"

import { ApplicationErrorComponent } from "../../../avail/_ui/x_preact/application_error"
import { LoadMenuEntry } from "../../../outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadBreadcrumbListComponent } from "../../../outline/_ui/action_load_breadcrumb_list/x_preact/load_breadcrumb_list"

import { DocsView, DocsResource } from "../resource"
import { DocsUsecaseContent } from "../../../../ui/vendor/getto-application/docs/data"

type EntryProps = Readonly<{
    view: DocsView
    docs: DocsUsecaseContent[]
}>
export function DocsUsecaseEntry(props: EntryProps): VNode {
    const resource = useApplicationView(props.view)

    const err = useNotifyUnexpectedError(resource)
    if (err) {
        return h(ApplicationErrorComponent, { err: `${err}` })
    }

    return h(DocsUsecaseComponent, { ...resource, docs: props.docs })
}

type Props = DocsResource & Readonly<{ docs: DocsUsecaseContent[] }>
export function DocsUsecaseComponent(resource: Props): VNode {
    useDocumentTitle(title())

    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([
                mainTitle(title()),
                h(LoadBreadcrumbListComponent, resource),
            ]),
            body: mainBody(resource.docs.map(content)),
            copyright,
        }),
        menu: h(LoadMenuEntry, resource),
    })

    function title() {
        if (resource.docs.length === 0) {
            return "no-title"
        }
        return resource.docs[0].title
    }
}

function content(docs: DocsUsecaseContent): VNode {
    return html`${[
        container(usecaseBox(docs)),
        container(docs.action.map(actionBox)),
        container(docs.data.map(dataBox)),
    ]}`
}
