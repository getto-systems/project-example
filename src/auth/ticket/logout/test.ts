import { test, expect } from "vitest"
import { observeApplicationState } from "../../../z_vendor/getto-application/action/test_helper"

import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { convertDB } from "../../../z_lib/ui/repository/init/convert"
import { authTicketRepositoryConverter } from "../kernel/convert"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { initLogoutAction, LogoutAction } from "./action"

test("logout", async () => {
    const { logout } = standard()

    expect(
        await observeApplicationState(logout.state, () => {
            return logout.submit()
        }),
    ).toEqual([{ type: "success" }])
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
        grantedRoles: ["role"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function standard_logoutRemote(): LogoutRemote {
    return async () => ({ success: true, value: true })
}
