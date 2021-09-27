import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"
import { toApplicationView } from "../../../../ui/vendor/getto-application/action/helper"

import { mockAuthenticatePasswordAction } from "../../password/action_authenticate/mock"
import { mockRequestResetTokenAction } from "../../password/reset/action_request_token/mock"
import { mockResetPasswordAction } from "../../password/reset/action_reset/mock"
import { mockCheckAuthTicketAction } from "../../ticket/action_check/mock"
import { mockSignViewLocationDetecter } from "../common/switch_view/mock"

import { initSignLinkResource } from "../common/nav/action_nav/init"

import { initSignAction } from "./init"

import { SignAction } from "./action"

describe("SignView", () => {
    test("redirect password authenticate", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            const state = await action.ignite()
            switch (state.type) {
                case "check-authTicket":
                    await state.view.resource.ignite()
            }
            return state
        }).then((stack) => {
            expect(stack.map((state) => state.type)).toEqual([
                "check-authTicket",
                "password-authenticate",
            ])
        })
    })

    test("static privacy policy", async () => {
        const { action } = static_privacyPolicy()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            const state = await action.ignite()
            switch (state.type) {
                case "check-authTicket":
                    await state.view.resource.ignite()
            }
            return state
        }).then((stack) => {
            expect(stack.map((state) => state.type)).toEqual([
                "check-authTicket",
                "static-privacyPolicy",
            ])
        })
    })

    test("password reset request token", async () => {
        const { action } = passwordReset_requestToken()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            const state = await action.ignite()
            switch (state.type) {
                case "check-authTicket":
                    await state.view.resource.ignite()
            }
            return state
        }).then((stack) => {
            expect(stack.map((state) => state.type)).toEqual([
                "check-authTicket",
                "password-reset-requestToken",
            ])
        })
    })

    test("password reset", async () => {
        const { action } = passwordReset_reset()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(async () => {
            const state = await action.ignite()
            switch (state.type) {
                case "check-authTicket":
                    await state.view.resource.ignite()
            }
            return state
        }).then((stack) => {
            expect(stack.map((state) => state.type)).toEqual(["check-authTicket", "password-reset"])
        })
    })

    test("error", async () => {
        const { action } = standard()

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => action.error("view error")).then((stack) => {
            expect(stack).toEqual([{ type: "error", err: "view error" }])
        })
    })

    test("terminate", async () => {
        const { action } = standard()
        const view = toApplicationView(action)

        const runner = setupActionTestRunner(action.subscriber)

        await runner(() => {
            view.terminate()
            return view.resource.error("view error")
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const currentURL = standard_URL()
    const action = initAction(currentURL)

    return { action }
}
function static_privacyPolicy() {
    const currentURL = static_privacyPolicy_URL()
    const action = initAction(currentURL)

    return { action }
}
function passwordReset_requestToken() {
    const currentURL = passwordReset_requestToken_URL()
    const action = initAction(currentURL)

    return { action }
}
function passwordReset_reset() {
    const currentURL = passwordReset_reset_URL()
    const action = initAction(currentURL)

    return { action }
}

function initAction(currentURL: URL): SignAction {
    return initSignAction(mockSignViewLocationDetecter(currentURL), {
        link: () => initSignLinkResource(),

        check: () => toApplicationView(mockCheckAuthTicketAction()),

        password_authenticate: () => toApplicationView(mockAuthenticatePasswordAction()),
        password_reset: () => toApplicationView(mockResetPasswordAction()),
        password_reset_requestToken: () => toApplicationView(mockRequestResetTokenAction()),
    })
}

function standard_URL(): URL {
    return new URL("https://example.com/index.html")
}
function static_privacyPolicy_URL(): URL {
    return new URL("https://example.com/index.html?-static=privacy-policy")
}
function passwordReset_requestToken_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset=request-token")
}
function passwordReset_reset_URL(): URL {
    return new URL("https://example.com/index.html?-password-reset=reset")
}
