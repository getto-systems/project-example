import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../src/x_outside_feature/common"

import { newDocsView } from "../../../../../src/docs/action_docs/init/resource"

import { docs_privacyPolicy } from "../../../../../src/docs/docs"

import { DocsDomainDetailEntry } from "../../../../../src/docs/action_docs/x_preact/domain_detail"

render(
    h(DocsDomainDetailEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_privacyPolicy,
    }),
    document.body,
)
