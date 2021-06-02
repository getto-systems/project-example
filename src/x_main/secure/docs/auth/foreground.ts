import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

import { newDocsView } from "../../../../docs/action_docs/init"

import { docs_auth, docs_auth_detail, docs_auth_summary } from "../../../../auth/docs"

import { DocsEntry } from "../../../../docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "認証・認可",
            contents: [[[...docs_auth, ...docs_auth_summary]], ...docs_auth_detail],
        },
    }),
    document.body,
)
