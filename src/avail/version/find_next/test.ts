import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../z_vendor/getto-application/action/test_helper"
import { standardApplicationTargetPath } from "./test_helper"

import { toApplicationView } from "../../../z_vendor/getto-application/action/helper"
import { ticker } from "../../../z_lib/ui/timer/helper"
import { mockFindNextVersionShell } from "./init/mock"

import { applicationPath } from "./helper"

import { CheckDeployExistsRemote } from "./infra"

import { FindNextVersionAction, initFindNextVersionAction } from "../find_next/action"
import { ApplicationView } from "../../../z_vendor/getto-application/action/action"

test("up to date", async () => {
    const { view } = standard()
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: true,
                version: "1.0.0-ui",
                target: {
                    valid: true,
                    value: {
                        specified: false,
                        path: "/index.html?search=parameter#hash",
                    },
                },
            },
        ])
    })
})

test("up to date; take longtime", async () => {
    const { view } = takeLongtime()
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            { type: "take-longtime" },
            {
                type: "success",
                upToDate: true,
                version: "1.0.0-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next major version", async () => {
    const { view } = found(["/2.0.0-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "2.0.0-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next minor version", async () => {
    const { view } = found(["/1.1.0-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.1.0-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next patch version", async () => {
    const { view } = found(["/1.0.1-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.0.1-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next minor version; recursive", async () => {
    const { view } = found(["/1.1.0-ui/index.html", "/1.2.0-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.2.0-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next patch version; recursive", async () => {
    const { view } = found(["/1.0.1-ui/index.html", "/1.0.2-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.0.2-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next patch version; complex", async () => {
    const { view } = found(["/1.1.0-ui/index.html", "/1.1.1-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.1.1-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next patch version; complex skipped", async () => {
    const { view } = found(["/1.1.0-ui/index.html", "/1.1.1-ui/index.html", "/1.1.3-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.1.1-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("found next minor version; complex current version", async () => {
    const { view } = foundComplex(["/1.1.0-ui/index.html"])
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: false,
                version: "1.1.0-ui",
                target: {
                    valid: true,
                    value: { specified: false, path: "/index.html?search=parameter#hash" },
                },
            },
        ])
    })
})

test("invalid version url", async () => {
    const { view } = invalidVersion()
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: true,
                version: "1.0.0-ui",
                target: { valid: false },
            },
        ])
    })
})

test("valid ApplicationTargetPath", () => {
    expect(
        applicationPath("1.0.0", {
            valid: true,
            value: standardApplicationTargetPath("/path/to/target.html"),
        }),
    ).toEqual("/1.0.0/path/to/target.html")
})

test("invalid ApplicationTargetPath", () => {
    expect(applicationPath("1.0.0", { valid: false })).toEqual("/1.0.0/index.html")
})

test("specify target", async () => {
    const { view } = specifyTarget()
    const resource = view.resource

    const runner = setupActionTestRunner(resource.subscriber)

    await runner(() => resource.ignitionState).then((stack) => {
        expect(stack).toEqual([
            {
                type: "success",
                upToDate: true,
                version: "1.0.0-ui",
                target: {
                    valid: true,
                    value: {
                        specified: true,
                        path: "/path/to/target.html?search=parameter#hash",
                    },
                },
            },
        ])
    })
})

test("terminate", async () => {
    const { view } = standard()

    const runner = setupActionTestRunner(view.resource.subscriber)

    await runner(() => {
        view.terminate()
        return view.resource.ignitionState
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
    })
})

function standard() {
    const view = initView(standard_URL(), standard_version(), standard_check())
    return { view }
}
function found(versions: string[]) {
    const view = initView(standard_URL(), standard_version(), found_check(versions))
    return { view }
}
function foundComplex(versions: string[]) {
    const view = initView(complex_URL(), complex_Version(), found_check(versions))
    return { view }
}
function invalidVersion() {
    const view = initView(invalidVersion_URL(), standard_version(), standard_check())
    return { view }
}
function takeLongtime() {
    const view = initView(standard_URL(), standard_version(), takeLongtime_check())
    return { view }
}
function specifyTarget() {
    const view = initView(specifyTarget_URL(), standard_version(), standard_check())
    return { view }
}

function initView(
    currentURL: URL,
    version: string,
    check: CheckDeployExistsRemote,
): ApplicationView<FindNextVersionAction> {
    return toApplicationView(
        initFindNextVersionAction({
            infra: {
                check,
            },
            shell: mockFindNextVersionShell(currentURL, version),
            config: {
                version,
                versionSuffix: "-ui",
                takeLongtimeThreshold: { wait_millisecond: 1 },
            },
        }),
    )
}

function standard_version(): string {
    return "1.0.0-ui"
}
function complex_Version(): string {
    return "1.0.0-rc1-ui"
}

function standard_URL(): URL {
    return new URL("https://example.com/1.0.0-ui/index.html?search=parameter#hash")
}
function complex_URL(): URL {
    return new URL("https://example.com/1.0.0-rc1-ui/index.html?search=parameter#hash")
}
function invalidVersion_URL(): URL {
    return new URL("https://example.com/invalid.html?search=parameter#hash")
}
function specifyTarget_URL(): URL {
    const path = encodeURIComponent("/path/to/target.html?search=parameter#hash")
    return new URL(`https://example.com/index.html?-application-target=${path}`)
}

function standard_check(): CheckDeployExistsRemote {
    return async () => ({ success: true, value: { found: false } })
}
function found_check(versions: string[]): CheckDeployExistsRemote {
    return async (version) => {
        return { success: true, value: { found: versions.includes(version) } }
    }
}
function takeLongtime_check(): CheckDeployExistsRemote {
    return async () =>
        ticker({ wait_millisecond: 2 }, () => ({ success: true, value: { found: false } }))
}
