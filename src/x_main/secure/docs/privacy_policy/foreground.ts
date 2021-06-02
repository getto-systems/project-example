import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

import { newDocsView } from "../../../../docs/action_docs/init"

import { docs_privacyPolicy } from "../../../../docs/docs"

import { DocsEntry } from "../../../../docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "プライバシーポリシー",
            contents: [[docs_privacyPolicy]],
        },
    }),
    document.body,
)
