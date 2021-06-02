import { ResetSessionID } from "../../../../password/reset/ui/data"
import { SignNavHref, SignNavItem } from "../data"

export type SignLinkResource = Readonly<{
    link: {
        getNav_static_privacyPolicy(): SignNavItem

        getNav_password_authenticate(): SignNavItem
        getNav_password_reset_requestToken(): SignNavItem
        getNav_password_reset_requestToken_retry(): SignNavItem
        getHref_password_reset_checkStatus(sessionID: ResetSessionID): SignNavHref
    }
}>
