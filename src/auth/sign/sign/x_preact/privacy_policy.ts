import { VNode } from "preact"

import { loginBox } from "../../../../z_vendor/getto-css/preact/layout/login"
import { buttons } from "../../../../z_vendor/getto-css/preact/design/form"

import { siteInfo } from "../../../../x_content/site"
import { signNav } from "../../nav/x_preact/nav"
import { docsActionField } from "../../../../docs/content/x_preact/helper"

import { docs_dataHandling } from "../../../../docs/docs"

import { SignLink } from "../../nav/action"

type Props = Readonly<{
    link: SignLink
}>
export function PrivacyPolicy(props: Props): VNode {
    // TODO プライバシーボリシーの内容は x_content の中に入れたい
    return loginBox(siteInfo, {
        title: "プライバシーポリシー",
        body: docs_dataHandling.action.map(docsActionField),
        footer: buttons({ left: loginLink(), right: resetLink() }),
    })

    function loginLink(): VNode {
        return signNav(props.link.getNav_password_authenticate())
    }
    function resetLink() {
        return signNav(props.link.getNav_password_reset_requestToken())
    }
}
