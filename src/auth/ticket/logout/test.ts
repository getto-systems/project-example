import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { AuthProfileRepository } from "../kernel/infra"

import { authProfileRepositoryConverter } from "../kernel/convert"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { AuthProfile } from "../kernel/data"
import { initLogoutAction, LogoutAction } from "./action"
import { LogoutRemote } from "./infra"

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
    const resource = initResource(standard_profileRepository())

    return { resource }
}

function initResource(
    profileRepository: AuthProfileRepository,
): Readonly<{ logout: LogoutAction }> {
    return {
        logout: initLogoutAction({
            profileRepository,
            logoutRemote: standard_logoutRemote(),
        }),
    }
}

function standard_profileRepository(): AuthProfileRepository {
    const result = authProfileRepositoryConverter.fromRepository({
        authAt: new Date("2020-01-01 09:00:00").toISOString(),
        roles: ["role"],
    })
    if (!result.valid) {
        throw new Error("invalid authn")
    }

    const repository = initMemoryDB<AuthProfile>()
    repository.set(result.value)
    return repository
}

function standard_logoutRemote(): LogoutRemote {
    return async () => ({ success: true, value: true })
}
