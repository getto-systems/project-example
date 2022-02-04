import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"

import { newDocsView } from "../../../../docs/content/init/resource"

import { DocsDomainDetailEntry } from "../../../../docs/content/x_preact/domain_detail"

import { docs_example } from "../../../../core/docs"

render(
    h(DocsDomainDetailEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_example,
    }),
    document.body,
)
