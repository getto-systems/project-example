import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../../../src/docs/action_docs/init/resource"

import { DocsEntry } from "../../../../../../../src/docs/action_docs/x_preact/docs"

import {
    docs_auth_sign,
    docs_auth_sign_description,
    docs_auth_sign_explanation,
    docs_auth_sign_negativeNote,
} from "../../../../../../../src/auth/_ui/action_sign/docs"

render(
    h(DocsEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: {
            title: "認証",
            contents: [
                [
                    [
                        ...docs_auth_sign,
                        ...docs_auth_sign_explanation,
                        ...docs_auth_sign_negativeNote,
                    ],
                ],
                ...docs_auth_sign_description,
            ],
        },
    }),
    document.body,
)
