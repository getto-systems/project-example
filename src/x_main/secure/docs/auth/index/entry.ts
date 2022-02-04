import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

import { newDocsView } from "../../../../../docs/content/init/resource"

import { DocsDomainEntry } from "../../../../../docs/content/x_preact/domain"

import { docs_auth } from "../../../../../auth/docs"

render(
    h(DocsDomainEntry, {
        view: newDocsView(newForegroundOutsideFeature()),
        docs: docs_auth,
    }),
    document.body,
)
