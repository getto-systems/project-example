import { detectMenuTargetPath, toMenuCategory, toMenuItem } from "../../kernel/convert"

import { LoadBreadcrumbListShell } from "../action"
import { BreadcrumbList } from "../data"

export function mockLoadBreadcrumbListShell(
    currentURL: URL,
    version: string,
): LoadBreadcrumbListShell {
    return {
        detectTargetPath: () => detectMenuTargetPath(currentURL, version),
    }
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
