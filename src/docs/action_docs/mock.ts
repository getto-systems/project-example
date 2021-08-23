import { mockNotifyUnexpectedErrorAction } from "../../avail/unexpected_error/_ui/action_notify/mock"
import {
    mockLoadBreadcrumbListAction,
    mockBreadcrumbList_home,
} from "../../example/outline/_ui/action_load_breadcrumb_list/mock"
import { mockLoadMenuResource } from "../../example/outline/_ui/action_load_menu/mock"

import { DocsResource } from "./resource"

export function mockDocsResource(): DocsResource {
    return {
        error: mockNotifyUnexpectedErrorAction(),
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        ...mockLoadMenuResource(),
    }
}
