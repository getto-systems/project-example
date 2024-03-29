import { test, expect } from "vitest"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"

import { newResetPasswordConfig } from "../../user/password/reset/reset/detail/config"
import { newAuthenticatePasswordConfig } from "../../user/password/authenticate/detail/config"
import { newCheckAuthTicketConfig } from "../../ticket/authenticate/detail/config"
import { newRequestResetTokenConfig } from "../../user/password/reset/request_token/detail/config"

import { mockResetPasswordShell } from "../../user/password/reset/reset/detail/mock"
import { mockGetScriptPathShell } from "../get_script_path/detail/mock"
import { mockRemoteInfraError } from "../../../common/util/remote/mock"
import { mockSignActionShell } from "./detail/mock"

import { initMemoryDB } from "../../../common/util/repository/detail/memory"
import { newClock } from "../../../common/util/clock/detail"

import { initSignAction, SignAction } from "./action"
import { initAuthenticateWithTokenAction } from "../../ticket/authenticate/action"
import { initAuthenticatePasswordAction } from "../../user/password/authenticate/action"
import { initResetPasswordAction } from "../../user/password/reset/reset/action"
import { initRequestResetTokenAction } from "../../user/password/reset/request_token/action"

import { AuthTicket } from "../../ticket/kernel/data"

test("redirect password authenticate", async () => {
    const { action } = standard(standard_URL())

    const result = observeAtom(action.state)

    const state = await action.state.ignitionState
    switch (state.type) {
        case "authTicket-check":
            await state.action.state.ignitionState
    }

    expect(result()).toMatchObject([
        { type: "authTicket-check" },
        { type: "password-authenticate" },
    ])
})

test("static privacy policy", async () => {
    const { action } = standard(static_privacyPolicy_URL())

    const result = observeAtom(action.state)

    const state = await action.state.ignitionState
    switch (state.type) {
        case "authTicket-check":
            await state.action.state.ignitionState
    }

    expect(result()).toMatchObject([{ type: "authTicket-check" }, { type: "static-privacyPolicy" }])
})

test("password reset request token", async () => {
    const { action } = standard(passwordReset_requestToken_URL())

    const result = observeAtom(action.state)

    const state = await action.state.ignitionState
    switch (state.type) {
        case "authTicket-check":
            await state.action.state.ignitionState
    }

    expect(result()).toMatchObject([
        { type: "authTicket-check" },
        { type: "password-reset-requestToken" },
    ])
})

test("password reset", async () => {
    const { action } = standard(passwordReset_reset_URL())

    const result = observeAtom(action.state)

    await action.state.ignitionState

    expect(result()).toMatchObject([{ type: "password-reset" }])
})

function standard(currentURL: URL) {
    const action = initAction(currentURL)
    return { action }
}

function initAction(currentURL: URL): SignAction {
    const ticketRepository = initMemoryDB<AuthTicket>()
    const renewRemote = async () => mockRemoteInfraError
    const authenticateRemote = async () => mockRemoteInfraError
    const resetRemote = async () => mockRemoteInfraError
    const requestTokenRemote = async () => mockRemoteInfraError
    const clock = newClock()

    return initSignAction(mockSignActionShell(currentURL), {
        check: () =>
            initAuthenticateWithTokenAction({
                infra: {
                    ticketRepository,
                    renewRemote,
                    clock,
                },
                shell: {
                    ...mockGetScriptPathShell(currentURL),
                },
                config: newCheckAuthTicketConfig(),
            }),

        password_authenticate: () =>
            initAuthenticatePasswordAction({
                infra: {
                    ticketRepository,
                    renewRemote,
                    authenticateRemote,
                    clock,
                },
                shell: {
                    ...mockGetScriptPathShell(currentURL),
                },
                config: newAuthenticatePasswordConfig(),
            }),
        password_reset: () =>
            initResetPasswordAction({
                infra: {
                    ticketRepository,
                    renewRemote,
                    resetRemote,
                    clock,
                },
                shell: mockResetPasswordShell(currentURL),
                config: newResetPasswordConfig(),
            }),
        password_reset_requestToken: () =>
            initRequestResetTokenAction({
                infra: {
                    requestTokenRemote,
                },
                config: newRequestResetTokenConfig(),
            }),
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
