import { setupActionTestRunner } from "../../../../../../ui/vendor/getto-application/action/test_helper"

import { mockRemotePod } from "../../../../../z_details/_ui/remote/mock"

import { mockCheckResetTokenSendingStatusLocationDetecter } from "../check_status/mock"

import { initCheckResetTokenSendingStatusView } from "./impl"
import {
    initCheckResetTokenSendingStatusCoreAction,
    initCheckResetTokenSendingStatusCoreMaterial,
} from "./core/impl"

import {
    GetResetTokenSendingStatusRemotePod,
    GetResetTokenSendingStatusResult,
    SendResetTokenRemotePod,
    SendResetTokenResult,
} from "../check_status/infra"

import { CheckResetTokenSendingStatusView } from "./resource"

import { ResetTokenSendingResult } from "../check_status/data"

describe("CheckPasswordResetSendingStatus", () => {
    test("valid session-id", async () => {
        const { view } = standard()
        const action = view.resource.checkStatus

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.ignite()).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-check-status" },
                { type: "succeed-to-send-token" },
            ])
        })
    })

    test("submit valid login-id; with long sending", async () => {
        // wait for send token check limit
        const { view } = takeLongtime()
        const action = view.resource.checkStatus

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.ignite()).then((stack) => {
            expect(stack).toEqual([
                { type: "try-to-check-status" },
                { type: "retry-to-check-status", status: { sending: true } },
                { type: "retry-to-check-status", status: { sending: true } },
                { type: "retry-to-check-status", status: { sending: true } },
                { type: "retry-to-check-status", status: { sending: true } },
                { type: "retry-to-check-status", status: { sending: true } },
                {
                    type: "failed-to-check-status",
                    err: { type: "infra-error", err: "overflow check limit" },
                },
            ])
        })
    })

    test("check without session id", async () => {
        const { view } = noSessionID()
        const action = view.resource.checkStatus

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.ignite()).then((stack) => {
            expect(stack).toEqual([
                { type: "failed-to-check-status", err: { type: "empty-session-id" } },
            ])
        })
    })

    test("terminate", async () => {
        const { view } = standard()
        const action = view.resource.checkStatus

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            view.terminate()
            return action.ignite()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const view = initView(standard_URL(), standard_sendToken(), standard_getStatus())

    return { view }
}
function takeLongtime() {
    const view = initView(standard_URL(), takeLongtime_sendToken(), takeLongtime_getStatus())

    return { view }
}
function noSessionID() {
    const view = initView(noSessionID_URL(), standard_sendToken(), standard_getStatus())

    return { view }
}

function initView(
    currentURL: URL,
    sendToken: SendResetTokenRemotePod,
    getStatus: GetResetTokenSendingStatusRemotePod,
): CheckResetTokenSendingStatusView {
    const checkStatusDetecter = mockCheckResetTokenSendingStatusLocationDetecter(currentURL)
    return initCheckResetTokenSendingStatusView(
        initCheckResetTokenSendingStatusCoreAction(
            initCheckResetTokenSendingStatusCoreMaterial(
                {
                    sendToken,
                    getStatus,
                    config: {
                        wait: { wait_millisecond: 32 },
                        limit: { limit: 5 },
                    },
                },
                checkStatusDetecter,
            ),
        ),
    )
}

function standard_URL() {
    return new URL(
        "https://example.com/index.html?-password-reset=check-status&-password-reset-session-id=session-id",
    )
}
function noSessionID_URL() {
    return new URL("https://example.com/index.html?-password-reset=check-status")
}

function standard_sendToken(): SendResetTokenRemotePod {
    return mockRemotePod(simulateSendToken, { wait_millisecond: 0 })
}
function takeLongtime_sendToken(): SendResetTokenRemotePod {
    return mockRemotePod(simulateSendToken, { wait_millisecond: 64 })
}

function standard_getStatus(): GetResetTokenSendingStatusRemotePod {
    return getStatusRemoteAccess([{ done: true, send: true }])
}
function takeLongtime_getStatus(): GetResetTokenSendingStatusRemotePod {
    // 完了するまでに 5回以上かかる
    return getStatusRemoteAccess([
        { done: false, status: { sending: true } },
        { done: false, status: { sending: true } },
        { done: false, status: { sending: true } },
        { done: false, status: { sending: true } },
        { done: false, status: { sending: true } },
        { done: false, status: { sending: true } },
    ])
}

function simulateSendToken(): SendResetTokenResult {
    return { success: true, value: true }
}
function getStatusRemoteAccess(
    responseCollection: ResetTokenSendingResult[],
): GetResetTokenSendingStatusRemotePod {
    let position = 0
    return mockRemotePod(
        (): GetResetTokenSendingStatusResult => {
            if (responseCollection.length === 0) {
                return { success: false, err: { type: "infra-error", err: "no response" } }
            }
            const response = getResponse()
            position++

            return { success: true, value: response }
        },
        { wait_millisecond: 0 },
    )

    function getResponse(): ResetTokenSendingResult {
        if (position < responseCollection.length) {
            return responseCollection[position]
        }
        return responseCollection[responseCollection.length - 1]
    }
}
