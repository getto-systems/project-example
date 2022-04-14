import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsView } from "../../../../docs/init/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { docs_auth_ticket } from "../../../../auth/ticket/docs"
import { docs_auth_user_loginId } from "../../../../auth/user/login_id/docs"
import { docs_auth_user_password } from "../../../../auth/user/password/docs"
import { docs_auth_user_account } from "../../../../auth/user/account/docs"

render(
    h(Docs, {
        view: newDocsView(newForegroundOutsideFeature()),
        title: "認証・認可",
        docs: [
            docs_auth_ticket,
            docs_auth_user_password,
            docs_auth_user_account,
            docs_auth_user_loginId,
        ],
    }),
    document.body,
)
