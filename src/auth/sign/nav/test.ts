import { initSignLink } from "./action"

describe("SignLink", () => {
    test("link", () => {
        const link = initSignLink()

        expect(link.getNav_static_privacyPolicy().href).toEqual("?-static=privacy-policy")
        expect(link.getNav_password_authenticate().href).toEqual(
            "?-password-authenticate=authenticate",
        )
        expect(link.getNav_password_reset_requestToken().href).toEqual(
            "?-password-reset=request-token",
        )
        expect(link.getNav_password_reset_requestToken_retry().href).toEqual(
            "?-password-reset=request-token",
        )
    })
})
