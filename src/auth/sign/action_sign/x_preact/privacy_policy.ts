import { VNode } from "preact"
import { html } from "htm/preact"

import { loginBox } from "../../../../../ui/vendor/getto-css/preact/layout/login"
import { buttons } from "../../../../../ui/vendor/getto-css/preact/design/form"

import { siteInfo } from "../../../../example/site"
import { signNav } from "../../nav/x_preact/nav"
import { docsSectionBody } from "../../../../docs/action_docs/x_preact/content"

import { docs_privacyPolicy } from "../../../../docs/docs"

import { SignLink } from "../../nav/resource"

type Props = Readonly<{
    link: SignLink
}>
export function PrivacyPolicyComponent(props: Props): VNode {
    return loginBox(siteInfo, {
        title: "プライバシーポリシー",
        body: docs_privacyPolicy.flatMap((section) => html`${section.body.map(docsSectionBody)}`),
        footer: buttons({ left: loginLink(), right: resetLink() }),
    })

    function loginLink(): VNode {
        return signNav(props.link.getNav_password_authenticate())
    }
    function resetLink() {
        return signNav(props.link.getNav_password_reset_requestToken())
    }
}
