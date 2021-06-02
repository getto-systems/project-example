import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

import { newDocsView } from "../../../../docs/action_docs/init"

import { docs_avail, docs_avail_detail } from "../../../../avail/docs"

import { DocsEntry } from "../../../../docs/action_docs/x_preact/docs"

render(
    h(DocsEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: {
            title: "保守・運用",
            contents: [[[...docs_avail]], [docs_avail_detail]],
        },
    }),
    document.body,
)
