import { initSignLinkResource } from "./init"

describe("SignLink", () => {
    test("link", () => {
        const resource = initSignLinkResource()

        expect(resource.link.getNav_static_privacyPolicy().href).toEqual("?-static=privacy-policy")
        expect(resource.link.getNav_password_authenticate().href).toEqual(
            "?-password-authenticate=authenticate",
        )
        expect(resource.link.getNav_password_reset_requestToken().href).toEqual(
            "?-password-reset=request-token",
        )
        expect(resource.link.getNav_password_reset_requestToken_retry().href).toEqual(
            "?-password-reset=request-token",
        )
    })
})
