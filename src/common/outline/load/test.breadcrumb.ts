import { test, expect } from "vitest"
import { standard_MenuTree } from "./test_helper"

import { mockLoadBreadcrumbListShell } from "./init/mock"

import { initLoadBreadcrumbListAction, LoadBreadcrumbListAction } from "./action"

test("load breadcrumb", () => {
    const { resource } = standard()

    expect(resource.breadcrumbList.load()).toEqual([
        category("MAIN"),
        item("ホーム", "home", "/1.0.0/index.html"),
    ])
})

test("load empty breadcrumb; unknown menu target", () => {
    const { resource } = unknownTarget()

    expect(resource.breadcrumbList.load()).toEqual([])
})

function category(label: string) {
    return { type: "category", category: { label } }
}
function item(label: string, icon: string, href: string) {
    return { type: "item", item: { label, icon, href } }
}

function standard() {
    const resource = newResource(standard_URL())

    return { resource }
}
function unknownTarget() {
    const resource = newResource(unknownTarget_URL())

    return { resource }
}

function newResource(currentURL: URL): Readonly<{ breadcrumbList: LoadBreadcrumbListAction }> {
    const version = standard_version()
    return {
        breadcrumbList: initLoadBreadcrumbListAction({
            shell: mockLoadBreadcrumbListShell(currentURL, version),
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