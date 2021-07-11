import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init"

import { docs_privacyPolicy } from "../../../../../src/docs/docs"

import { DocsEntry } from "../../../../../src/docs/action_docs/x_preact/docs"

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
