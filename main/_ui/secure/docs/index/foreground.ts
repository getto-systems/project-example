import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init"

import { docs_example } from "../../../../../src/example/docs"
import { docs_avail_legacy } from "../../../../../src/avail/docs"
import { docs_docs } from "../../../../../src/docs/docs"
import { docs_auth_legacy } from "../../../../../src/auth/docs"

import { DocsEntry } from "../../../../../src/docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "ドキュメント",
            contents: [[docs_example], [[...docs_docs, ...docs_avail_legacy, ...docs_auth_legacy]]],
        },
    }),
    document.body,
)
