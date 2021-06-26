import { SignNavItem } from "../data"

export type SignLinkResource = Readonly<{
    link: {
        getNav_static_privacyPolicy(): SignNavItem

        getNav_password_authenticate(): SignNavItem
        getNav_password_reset_requestToken(): SignNavItem
        getNav_password_reset_requestToken_retry(): SignNavItem
    }
}>
