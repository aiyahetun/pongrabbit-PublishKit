export type UiLocale = "zh-CN" | "en";



export type MarkdownScanItem = {

  path: string;

  title: string;

  sizeBytes: number;

  format: string;

};



export type WorkspaceSettings = {

  uiLocale: UiLocale;

  copyRoot?: string;

  mediaRoot?: string;

  videoRoot?: string;

  apiPort?: number;

  pairingToken?: string;

  projectName?: string;

  brandDomestic?: string;

  brandOverseas?: string;

  scanIgnoreDirs?: string[];

  licenseKey?: string;

  onboardingDone?: boolean;

};



export type LicenseStatus = {

  tier: "free" | "pro" | string;

  contentCount: number;

  contentLimit: number;

  isPro: boolean;

};



export type ApiStatus = {

  port: number;

  pairingToken: string;

  baseUrl: string;

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



export type ImportBackupResult = {

  mode: string;

  fileCount: number;

  includesThumbs: boolean;

  merged?: MergeBackupSummary;

};



export type MergeBackupSummary = {

  sourceDocumentsAdded: number;

  contentItemsAdded: number;

  publishTasksAdded: number;

  mediaAssetsAdded: number;

  contentMediaAdded: number;

  channelsAdded: number;

};



export type DuplicatePublishWarning = {

  previousPublishedAt: string;

  previousPublishUrl: string;

  daysSince: number;

};



export type ImportSplitsResult = {

  importedCount: number;

  contentIds: string[];

};



export type TableRowPreview = {

  index: number;

  title: string;

  body: string;

  language: string;

  channelName?: string;

  bodyPreview: string;

  recommended: boolean;

};



export type TableImportPreview = {

  columns: string[];

  titleColumn: string;

  bodyColumn: string;

  languageColumn?: string;

  channelColumn?: string;

  rows: TableRowPreview[];

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

  blockedReason?: string;

  checklist?: string[];

  updatedAt: string;

  scheduledAt?: string;

  publishedAt?: string;

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



export type MediaAsset = {

  id: string;

  path: string;

  fileName: string;

  kind: string;

  sizeBytes: number;

  indexedAt: string;

  thumbPath?: string;

};



export type MediaSuggestion = MediaAsset & {

  reason: string;

  score: number;

};



export type ExportContentPackResult = {

  folderPath: string;

  mediaCount: number;

};



export type ExportTasksCsvResult = {

  path: string;

  rowCount: number;

};



export type StageImagesResult = {

  folderPath: string;

  copiedCount: number;

  isTemporary?: boolean;

};



export type DeleteContentResult = {

  deletedCount: number;

};



export type ExportBackupResult = {

  path: string;

  fileCount: number;

  includesThumbs: boolean;

};



export type ScanMediaResult = {

  indexedCount: number;

  totalCount: number;

};



export type CalendarEntry = {

  id: string;

  status: string;

  date: string;

  channelName: string;

  channelColor: string;

  contentTitle: string;

  publishUrl: string;

};

