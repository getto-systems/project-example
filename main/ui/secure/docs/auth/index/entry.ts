import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/common"

import { newDocsView } from "../../../../../../src/docs/action_docs/init/resource"

import { DocsDomainEntry } from "../../../../../../src/docs/action_docs/x_preact/domain"

import { docs_auth } from "../../../../../../src/auth/docs"

render(
    h(DocsDomainEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_auth,
    }),
    document.body,
)
