import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"
import { toApplicationView } from "../../../../ui/vendor/getto-application/action/helper"

import { newResetPasswordConfig } from "../../user/password/reset/reset/init/config"
import { newAuthenticatePasswordConfig } from "../../user/password/authenticate/init/config"
import { newCheckAuthTicketConfig } from "../../ticket/check/init/config"

import { mockRequestResetTokenAction } from "../../user/password/reset/action_request_token/mock"
import { mockSignViewLocationDetecter } from "../router/mock"
import { mockResetPasswordShell } from "../../user/password/reset/reset/init/mock"
import { mockGetScriptPathShell } from "../get_script_path/mock"
import { mockRemoteInfraError } from "../../../z_lib/ui/remote/mock"

import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { newClock } from "../../../z_lib/ui/clock/init"
import { initSignLinkResource } from "../action_nav/init"

import { initSignAction } from "./init"
import { SignAction } from "./action"
import { initCheckAuthTicketAction } from "../../ticket/check/action"
import { initAuthenticatePasswordAction } from "../../user/password/authenticate/action"
import { initResetPasswordAction } from "../../user/password/reset/reset/action"

import { AuthTicket } from "../../ticket/kernel/data"

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
            return await action.ignite()
        }).then((stack) => {
            expect(stack.map((state) => state.type)).toEqual(["password-reset"])
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
    const ticketRepository = initMemoryDB<AuthTicket>()
    const renewRemote = async () => mockRemoteInfraError
    const authenticateRemote = async () => mockRemoteInfraError
    const resetRemote = async () => mockRemoteInfraError
    const clock = newClock()

    return initSignAction(mockSignViewLocationDetecter(currentURL), {
        link: () => initSignLinkResource(),

        check: () =>
            toApplicationView(
                initCheckAuthTicketAction(
                    newCheckAuthTicketConfig(),
                    {
                        ticketRepository,
                        renewRemote,
                        clock,
                    },
                    {
                        ...mockGetScriptPathShell(currentURL),
                    },
                ),
            ),

        password_authenticate: () =>
            toApplicationView(
                initAuthenticatePasswordAction(
                    newAuthenticatePasswordConfig(),
                    {
                        ticketRepository,
                        renewRemote,
                        authenticateRemote,
                        clock,
                    },
                    {
                        ...mockGetScriptPathShell(currentURL),
                    },
                ),
            ),
        password_reset: () =>
            toApplicationView(
                initResetPasswordAction(
                    newResetPasswordConfig(),
                    {
                        ticketRepository,
                        renewRemote,
                        resetRemote,
                        clock,
                    },
                    mockResetPasswordShell(currentURL),
                ),
            ),
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
