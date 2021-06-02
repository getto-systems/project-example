import { toMenuCategory, toMenuItem } from "../../kernel/impl/converter"

import { LoadBreadcrumbListCoreAction } from "./action"

import { BreadcrumbList } from "../../load_breadcrumb_list/data"

export function mockLoadBreadcrumbListCoreAction(
    breadcrumbList: BreadcrumbList,
): LoadBreadcrumbListCoreAction {
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
