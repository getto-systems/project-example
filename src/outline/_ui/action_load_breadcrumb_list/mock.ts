import { mockLoadBreadcrumbListCoreAction, mockBreadcrumbList_home } from "./core/mock"

import { LoadBreadcrumbListResource } from "./resource"

export function mockLoadBreadcrumbListResource(): LoadBreadcrumbListResource {
    return {
        breadcrumbList: mockLoadBreadcrumbListCoreAction(mockBreadcrumbList_home()),
    }
}
