import { markResetSessionID } from "../../../../password/reset/ui/test_helper"

import { initSignLinkResource } from "./impl"

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
        expect(resource.link.getHref_password_reset_checkStatus(markResetSessionID("session-id"))).toEqual(
            "?-password-reset=check-status&-password-reset-session-id=session-id",
        )
    })
})
