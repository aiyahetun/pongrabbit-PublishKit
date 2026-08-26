export type NavItem = {
  to: string;
  labelKey: string;
  hintKey: string;
  step?: number;
};

export type NavGroup = {
  id: string;
  titleKey: string;
  items: NavItem[];
};

export const NAV_GROUPS: NavGroup[] = [
  {
    id: "prepare",
    titleKey: "nav.groupPrepare",
    items: [
      { to: "/sources", labelKey: "nav.sources", hintKey: "nav.hint.sources", step: 1 },
      { to: "/content", labelKey: "nav.content", hintKey: "nav.hint.content", step: 2 },
      { to: "/media", labelKey: "nav.media", hintKey: "nav.hint.media", step: 3 },
    ],
  },
  {
    id: "publish",
    titleKey: "nav.groupPublish",
    items: [
      { to: "/tasks", labelKey: "nav.tasks", hintKey: "nav.hint.tasks", step: 4 },
      { to: "/today", labelKey: "nav.today", hintKey: "nav.hint.today", step: 5 },
      { to: "/calendar", labelKey: "nav.calendar", hintKey: "nav.hint.calendar", step: 6 },
    ],
  },
  {
    id: "workspace",
    titleKey: "nav.groupWorkspace",
    items: [{ to: "/settings", labelKey: "nav.settings", hintKey: "nav.hint.settings" }],
  },
];

export const NAV_COLLAPSE_STORAGE_KEY = "publishkit.nav.collapsed.v1";
