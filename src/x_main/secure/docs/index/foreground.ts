import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

import { newDocsView } from "../../../../docs/action_docs/init"

import { docs_example } from "../../../../example/docs"
import { docs_avail } from "../../../../avail/docs"
import { docs_docs } from "../../../../docs/docs"
import { docs_auth } from "../../../../auth/docs"

import { DocsEntry } from "../../../../docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "ドキュメント",
            contents: [[docs_example], [[...docs_docs, ...docs_avail, ...docs_auth]]],
        },
    }),
    document.body,
)
