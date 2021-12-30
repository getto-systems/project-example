import { setupActionTestRunner } from "../../../../ui/vendor/getto-application/action/test_helper"

import { initLogoutAction, initLogoutMaterial } from "./init"

import { AuthProfileRepository } from "../kernel/infra"
import { LogoutRemote } from "../logout/infra"

import { LogoutResource } from "./resource"

import { authProfileRepositoryConverter } from "../kernel/convert"
import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { AuthProfile } from "../kernel/data"

describe("Logout", () => {
    test("clear", async () => {
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
    const resource = initResource(standard_profile_repository())

    return { resource }
}

function initResource(profileRepository: AuthProfileRepository): LogoutResource {
    return {
        logout: initLogoutAction(
            initLogoutMaterial({
                profileRepository,
                logoutRemote: standard_logout_remote(),
            }),
        ),
    }
}

function standard_profile_repository(): AuthProfileRepository {
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

function standard_logout_remote(): LogoutRemote {
    return async () => ({ success: true, value: true })
}
