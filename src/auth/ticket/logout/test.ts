import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"

import { convertDB } from "../../../z_lib/ui/repository/init/convert"
import { authTicketRepositoryConverter } from "../kernel/convert"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../kernel/infra"
import { LogoutRemote } from "./infra"

import { initLogoutAction, LogoutAction } from "./action"

describe("Logout", () => {
    test("logout", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.logout.subscriber)

        await runner(() => resource.logout.submit()).then((stack) => {
            expect(stack).toEqual([{ type: "succeed-to-logout" }])
        })
    })

    test("terminate", async () => {
        const { resource } = standard()

        const runner = setupActionTestRunner(resource.logout.subscriber)

        await runner(() => {
            resource.logout.terminate()
            return resource.logout.submit()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    const resource = initResource()

    return { resource }
}

function initResource(): Readonly<{ logout: LogoutAction }> {
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
        roles: ["role"],
    })
    return convertDB(db, authTicketRepositoryConverter)
}

function standard_logoutRemote(): LogoutRemote {
    return async () => ({ success: true, value: true })
}