import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"

import { mockRepository } from "../../../../z_details/_ui/repository/mock"
import { mockAuthnRepository } from "../kernel/infra/repository/mock"

import { convertRepository } from "../../../../z_details/_ui/repository/helper"
import { initLogoutCoreAction, initLogoutCoreMaterial } from "./core/impl"
import { initLogoutResource } from "./impl"

import { AuthnRepository, AuthzRepositoryPod, AuthzRepositoryValue } from "../kernel/infra"
import { ClearAuthTicketRemote } from "../clear/infra"

import { LogoutResource } from "./resource"

import { authnRepositoryConverter } from "../kernel/converter"

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
    const resource = initResource(standard_authn(), standard_authz())

    return { resource }
}

function initResource(authn: AuthnRepository, authz: AuthzRepositoryPod): LogoutResource {
    return initLogoutResource(
        initLogoutCoreAction(
            initLogoutCoreMaterial({
                authn,
                authz,
                clear: standard_clear(),
            }),
        ),
    )
}

function standard_authn(): AuthnRepository {
    const result = authnRepositoryConverter.fromRepository({
        authAt: new Date("2020-01-01 09:00:00").toISOString(),
    })
    if (!result.valid) {
        throw new Error("invalid authn")
    }

    const repository = mockAuthnRepository()
    repository.set(result.value)
    return repository
}
function standard_authz(): AuthzRepositoryPod {
    const db = mockRepository<AuthzRepositoryValue>()
    db.set({
        roles: ["role"],
    })
    return convertRepository(db)
}

function standard_clear(): ClearAuthTicketRemote {
    return async () => ({ success: true, value: true })
}
