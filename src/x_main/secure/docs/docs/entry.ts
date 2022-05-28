import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsResource } from "../../../../docs/init/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { docs_docs } from "../../../../docs/docs"

render(
    h(Docs, {
        ...newDocsResource(newForegroundOutsideFeature()),
        title: "ドキュメント",
        docs: [docs_docs],
    }),
    document.body,
)
