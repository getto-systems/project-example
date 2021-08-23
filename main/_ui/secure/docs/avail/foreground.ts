import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init/resource"

import { DocsDomainDetailEntry } from "../../../../../src/docs/action_docs/x_preact/domain_detail"

import { docsDomainContent } from "../../../../../ui/vendor/getto-application/docs/helper"
import { docs_avail } from "../../../../../src/avail/docs"

render(
    h(DocsDomainDetailEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: docsDomainContent(docs_avail),
    }),
    document.body,
)
