import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init"

import { docs_avail_legacy, docs_avail_detail } from "../../../../../src/avail/docs"

import { DocsEntry } from "../../../../../src/docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "保守・運用",
            contents: [[[...docs_avail_legacy]], [docs_avail_detail]],
        },
    }),
    document.body,
)
