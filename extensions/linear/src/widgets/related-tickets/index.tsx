import { WidgetGroup } from "@hypr/extension-types";

import Widget2x2 from "./2x2";
import WidgetFull from "./full";

export default {
  id: "related-tickets",
  items: [
    {
      type: "twoByTwo",
      component: Widget2x2,
    },
    {
      type: "full",
      component: WidgetFull,
    },
  ],
} satisfies WidgetGroup;
