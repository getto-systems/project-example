import { mockNotifyUnexpectedErrorAction } from "../../avail/unexpected_error/action_notify/mock"
import {
    mockLoadBreadcrumbListAction,
    mockBreadcrumbList_home,
} from "../../example/outline/action_load_breadcrumb_list/mock"
import { mockLoadMenuAction, mockMenu_home } from "../../example/outline/action_load_menu/mock"

import { DocsResource } from "./resource"

export function mockDocsResource(): DocsResource {
    return {
        error: mockNotifyUnexpectedErrorAction(),
        breadcrumbList: mockLoadBreadcrumbListAction(mockBreadcrumbList_home()),
        menu: mockLoadMenuAction(mockMenu_home()),
    }
}
