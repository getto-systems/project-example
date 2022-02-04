import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newDocsView } from "../../../../docs/content/init/resource"

import { docs_privacyPolicy } from "../../../../docs/docs"

import { DocsDomainDetailEntry } from "../../../../docs/content/x_preact/domain_detail"

render(
    h(DocsDomainDetailEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_privacyPolicy,
    }),
    document.body,
)
