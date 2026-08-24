export type UiLocale = "zh-CN" | "en";



export type MarkdownScanItem = {

  path: string;

  title: string;

  sizeBytes: number;

};



export type WorkspaceSettings = {

  uiLocale: UiLocale;

  copyRoot?: string;

  mediaRoot?: string;

};



export type SplitStrategy = "whole" | "h1" | "h2" | "h3" | "smart";



export type SectionKind = "content" | "meta" | "platform";



export type SplitPreview = {

  index: number;

  title: string;

  heading?: string;

  startLine: number;

  endLine: number;

  bodyPreview: string;

  body: string;

  language: string;

  sectionKind: SectionKind;

  recommended: boolean;

};



export type ContentItem = {

  id: string;

  title: string;

  sourcePath: string;

  language: string;

  body: string;

  createdAt: string;

};



export type ImportSplitsResult = {

  importedCount: number;

  contentIds: string[];

};



export type Channel = {

  id: string;

  name: string;

  market: string;

  color: string;

  isCustom: boolean;

};



export type TaskStatus = "draft" | "ready" | "scheduled" | "published" | "archived" | "blocked";



export type PublishTask = {

  id: string;

  status: TaskStatus | string;

  publishUrl: string;

  note: string;

  updatedAt: string;

  channel: {

    id: string;

    name: string;

    color: string;

  };

  content: {

    id: string;

    title: string;

    language: string;

    body: string;

  };

};

