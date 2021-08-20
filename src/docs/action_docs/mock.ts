import { mockNotifyUnexpectedErrorResource } from "../../avail/unexpected_error/_ui/action_notify/mock"
import { mockLoadBreadcrumbListResource } from "../../outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuResource } from "../../outline/_ui/action_load_menu/mock"

import { DocsResource } from "./resource"

export function mockDocsResource(): DocsResource {
    return {
        ...mockNotifyUnexpectedErrorResource(),
        ...mockLoadBreadcrumbListResource(),
        ...mockLoadMenuResource(),
    }
}
