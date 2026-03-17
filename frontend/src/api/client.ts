export async function fetchHealth(): Promise<{ status: string; service: string }> {
  const response = await fetch("/healthz");
  if (!response.ok) {
    throw new Error(`Health check failed with status ${response.status}`);
  }

  return response.json();
}

export async function fetchSite() {
  const response = await fetch("/api/site");
  if (!response.ok) {
    throw new Error(`Site request failed with status ${response.status}`);
  }

  return response.json();
}

export async function generateStaging() {
  const response = await fetch("/api/staging/generate", {
    method: "POST"
  });
  if (!response.ok) {
    throw new Error(`Staging request failed with status ${response.status}`);
  }

  return response.json();
}
