import { toMenuCategory, toMenuItem } from "../kernel/convert"

import { LoadBreadcrumbListAction } from "./action"

import { BreadcrumbList } from "../load_breadcrumb_list/data"

export function mockLoadBreadcrumbListAction(
    breadcrumbList: BreadcrumbList,
): LoadBreadcrumbListAction {
    return {
        load: () => breadcrumbList,
    }
}

export function mockBreadcrumbList_home(): BreadcrumbList {
    return mockBreadcrumbList("ホーム")
}
export function mockBreadcrumbList(label: string): BreadcrumbList {
    return [
        {
            type: "category",
            category: toMenuCategory({ label: "MAIN", permission: { type: "allow" } }),
        },
        {
            type: "item",
            item: toMenuItem({ icon: "home", label, path: "#" }, "1.0.0"),
        },
    ]
}
