export function humanizeScopedName(siteName: string, value: string): string {
  const sitePrefix = `${siteName.toLowerCase()}-`;
  return value.toLowerCase().startsWith(sitePrefix) ? value.slice(sitePrefix.length) : value;
}
