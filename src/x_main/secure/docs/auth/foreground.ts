import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

import { newDocsView } from "../../../../docs/action_docs/init"

import { DocsDomainEntry } from "../../../../docs/action_docs/x_preact/domain"

import { docsDomainContent } from "../../../../../ui/vendor/getto-application/docs/helper"
import { docs_auth } from "../../../../auth/docs"

render(
    h(DocsDomainEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: docsDomainContent(docs_auth),
    }),
    document.body,
)
