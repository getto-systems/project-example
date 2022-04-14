import { VNode } from "preact"
import { html } from "htm/preact"

import { loginBox } from "../../../../z_vendor/getto-css/preact/layout/login"
import { buttons, field } from "../../../../z_vendor/getto-css/preact/design/form"

import { content_privacyPolicy } from "../../../../x_content/privacy_policy"
import { siteInfo } from "../../../../x_content/site"
import { signNav } from "../../nav/x_preact/nav"

import { SignLink } from "../../nav/action"

type Props = Readonly<{
    link: SignLink
}>
export function PrivacyPolicy(props: Props): VNode {
    return loginBox(siteInfo, {
        title: "プライバシーポリシー",
        body: content_privacyPolicy.descriptions.map((description) =>
            field({
                title: description.title,
                body: html`<ul>
                    ${description.description.map((description) => html`<li>${description}</li>`)}
                </ul>`,
            }),
        ),
        footer: buttons({ left: loginLink(), right: resetLink() }),
    })

    function loginLink(): VNode {
        return signNav(props.link.getNav_password_authenticate())
    }
    function resetLink() {
        return signNav(props.link.getNav_password_reset_requestToken())
    }
}
