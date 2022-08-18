import { test, expect } from "vitest"
import { standard_MenuTree } from "./test_helper"

import { mockOutlineBreadcrumbListShell } from "./init/mock"

import { initOutlineBreadcrumbListAction, OutlineBreadcrumbListAction } from "./action"

test("load breadcrumb", () => {
    const { breadcrumbList } = standard()

    expect(breadcrumbList.state.currentState()).toEqual({
        list: [category("MAIN"), item("ホーム", "home", "/1.0.0/index.html")],
    })
})

test("load empty breadcrumb; unknown menu target", () => {
    const { breadcrumbList } = unknownTarget()

    expect(breadcrumbList.state.currentState()).toEqual({ list: [] })
})

function category(label: string) {
    return { type: "category", category: { label } }
}
function item(label: string, icon: string, href: string) {
    return { type: "item", item: { label, icon, href } }
}

function standard() {
    return newResource(standard_URL())
}
function unknownTarget() {
    return newResource(unknownTarget_URL())
}

function newResource(currentURL: URL): Readonly<{ breadcrumbList: OutlineBreadcrumbListAction }> {
    const version = standard_version()
    return {
        breadcrumbList: initOutlineBreadcrumbListAction({
            shell: mockOutlineBreadcrumbListShell(currentURL, version),
            config: {
                version,
                menuTree: standard_MenuTree(),
            },
        }),
    }
}

function standard_version(): string {
    return "1.0.0"
}

function standard_URL(): URL {
    return new URL("https://example.com/1.0.0/index.html")
}
function unknownTarget_URL(): URL {
    return new URL("https://example.com/1.0.0/unknown.html")
}
