import { render, h } from "preact"

import { newForegroundOutsideFeature } from "../../../../x_outside_feature/common"
import { newDocsResource } from "../../../../docs/detail/resource"

import { Docs } from "../../../../docs/x_preact/docs"

import { content_privacyPolicy } from "../../../../x_content/privacy_policy"

render(
    h(Docs, {
        ...newDocsResource(newForegroundOutsideFeature()),
        title: "プライバシーポリシー",
        docs: [content_privacyPolicy],
    }),
    document.body,
)
