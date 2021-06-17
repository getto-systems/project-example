import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"

import { mockAuthnRepository, mockAuthzRepository } from "../kernel/infra/repository/mock"

import { initLogoutCoreAction, initLogoutCoreMaterial } from "./core/impl"
import { initLogoutResource } from "./impl"

import { AuthnRepository, AuthzRepository } from "../kernel/infra"
import { LogoutRemote } from "../logout/infra"

import { LogoutResource } from "./resource"

import { authnRepositoryConverter, authzRepositoryConverter } from "../kernel/convert"

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

function initResource(authn: AuthnRepository, authz: AuthzRepository): LogoutResource {
    return initLogoutResource(
        initLogoutCoreAction(
            initLogoutCoreMaterial({
                authn,
                authz,
                logout: standard_clear(),
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
function standard_authz(): AuthzRepository {
    const result = authzRepositoryConverter.fromRepository({
        roles: ["role"],
    })
    if (!result.valid) {
        throw new Error("invalid authz")
    }

    const repository = mockAuthzRepository()
    repository.set(result.value)
    return repository
}

function standard_clear(): LogoutRemote {
    return async () => ({ success: true, value: true })
}
