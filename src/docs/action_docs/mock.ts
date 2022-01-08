import { mockLoadMenuAction, mockMenu_home } from "../../example/outline/action_load_menu/mock"
import { mockBreadcrumbList } from "../../example/outline/load_breadcrumb_list/init/mock"

import { DocsResource } from "./resource"

export function mockDocsResource(): DocsResource {
    return {
        error: { notify: () => null },
        breadcrumbList: { load: () => mockBreadcrumbList("ホーム") },
        menu: mockLoadMenuAction(mockMenu_home()),
    }
}
