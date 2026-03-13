import { useEffect, useState } from "react";
import { fetchSite } from "../api/client";
import { mockSite } from "../data/mockSite";
import { SiteViewModel } from "../types/site";

interface SiteDataState {
  site: SiteViewModel;
  source: "api" | "mock";
  loading: boolean;
  error: string | null;
}

export function useSiteData(): SiteDataState {
  const [state, setState] = useState<SiteDataState>({
    site: mockSite,
    source: "mock",
    loading: true,
    error: null
  });

  useEffect(() => {
    let active = true;

    async function load() {
      try {
        const site = (await fetchSite()) as SiteViewModel;
        if (!active) {
          return;
        }

        setState({
          site,
          source: "api",
          loading: false,
          error: null
        });
      } catch (error) {
        if (!active) {
          return;
        }

        setState({
          site: mockSite,
          source: "mock",
          loading: false,
          error: error instanceof Error ? error.message : "Failed to load site data"
        });
      }
    }

    void load();

    return () => {
      active = false;
    };
  }, []);

  return state;
}
