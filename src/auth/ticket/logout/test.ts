import { test, expect } from "vitest"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"

import { initMemoryDB } from "../../../common/util/repository/detail/memory"

import { convertDB } from "../../../common/util/repository/detail/convert"
import { authTicketRepositoryConverter } from "../kernel/convert"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { initLogoutAction, LogoutAction } from "./action"

test("logout", async () => {
    const { logout } = standard()

    const result = observeAtom(logout.state)

    await logout.submit()

    expect(result()).toEqual([{ type: "success" }])
})

function standard(): Readonly<{ logout: LogoutAction }> {
    return {
        logout: initLogoutAction({
            ticketRepository: standard_ticketRepository(),
            logoutRemote: standard_logoutRemote(),
        }),
    }
}

function standard_ticketRepository(): AuthTicketRepository {
    const db = initMemoryDB<AuthTicketRepositoryValue>()
    db.set({
        authAt: "2020-01-01 09:00:00",
        granted: ["permission"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function standard_logoutRemote(): LogoutRemote {
    return async () => ({ success: true, value: true })
}
