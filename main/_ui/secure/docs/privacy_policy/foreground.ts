import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init/resource"

import { docs_privacyPolicy } from "../../../../../src/docs/docs"

import { DocsEntry } from "../../../../../src/docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: {
            title: "プライバシーポリシー",
            contents: [[docs_privacyPolicy]],
        },
    }),
    document.body,
)
