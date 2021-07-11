import { render, h } from "preact"

import { foregroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"

import { newDocsView } from "../../../../../../src/docs/action_docs/init"

import { DocsDomainEntry } from "../../../../../../src/docs/action_docs/x_preact/domain"

import { docsDomainContent } from "../../../../../../ui/vendor/getto-application/docs/helper"
import { docs_auth } from "../../../../../../src/auth/docs"

render(
    h(DocsDomainEntry, {
        view: newDocsView(foregroundOutsideFeature()),
        docs: docsDomainContent(docs_auth),
    }),
    document.body,
)
