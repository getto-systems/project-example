import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init/resource"

import { DocsDomainDetailEntry } from "../../../../../src/docs/action_docs/x_preact/domain_detail"

import { docs_avail } from "../../../../../src/avail/docs"

render(
    h(DocsDomainDetailEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_avail,
    }),
    document.body,
)
